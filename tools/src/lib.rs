use std::{cell::RefCell, rc::Rc, sync::Arc};

use deno_core::*;
use serde::{Deserialize, Serialize};

// 请求配置
#[derive(Debug, Deserialize)]
struct RequestConfig {
    url: String,
    method: Option<String>,
    headers: Option<serde_json::Value>,
    body: Option<String>,
    timeout: Option<u64>, // 超时时间（毫秒）
}

// 响应结构
#[derive(Debug, Serialize)]
struct ResponseData {
    ok: bool,
    status: u16,
    headers: serde_json::Value,
    data: serde_json::Value,
}

// 错误响应
#[derive(Debug, Serialize)]
struct ErrorResponse {
    ok: bool,
    error: String,
    status: Option<u16>,
}

#[op2(async)]
#[serde]
async fn op_fetch(
    #[serde] config: RequestConfig,
) -> Result<serde_json::Value, deno_error::JsErrorBox> {
    // 创建请求客户端
    let client = reqwest::Client::new();

    // 构建请求
    let mut request_builder = match config.method {
        Some(method) => client.request(
            reqwest::Method::from_bytes(method.as_bytes()).unwrap_or(reqwest::Method::GET),
            &config.url,
        ),
        None => client.get(&config.url),
    };

    // 添加请求头
    if let Some(headers) = config.headers {
        if let Some(headers_obj) = headers.as_object() {
            for (key, value) in headers_obj {
                if let Some(value_str) = value.as_str() {
                    request_builder = request_builder.header(key, value_str);
                }
            }
        }
    }

    // 添加请求体
    if let Some(body) = config.body {
        request_builder = request_builder.body(body);
    }

    // 设置超时
    let timeout = config.timeout.unwrap_or(30000);
    request_builder = request_builder.timeout(std::time::Duration::from_millis(timeout));

    // 发送请求
    let response = match request_builder.send().await {
        Ok(resp) => resp,
        Err(e) => {
            let error_resp =
                ErrorResponse { ok: false, error: format!("请求失败: {}", e), status: None };
            return Ok(serde_json::to_value(error_resp).unwrap());
        }
    };

    // 获取响应状态和头信息
    let status_code = response.status();
    let status = response.status().as_u16();
    let headers = response
        .headers()
        .iter()
        .map(|(name, value)| {
            (
                name.as_str().to_string(),
                serde_json::Value::String(value.to_str().unwrap_or("").to_string()),
            )
        })
        .collect::<serde_json::Map<String, serde_json::Value>>();

    // 获取响应数据
    let data = match response.text().await {
        Ok(text) => {
            // 尝试解析为 JSON
            serde_json::from_str(&text).unwrap_or_else(|_| {
                // 如果不是 JSON，返回文本
                serde_json::Value::String(text)
            })
        }
        Err(e) => {
            let error_resp = ErrorResponse {
                ok: false,
                error: format!("读取响应失败: {}", e),
                status: Some(status),
            };
            return Ok(serde_json::to_value(error_resp).unwrap());
        }
    };

    // 构建响应
    let response_data = ResponseData {
        ok: status_code.is_success(),
        status,
        headers: serde_json::Value::Object(headers),
        data,
    };

    Ok(serde_json::to_value(response_data).unwrap())
}

#[derive(Serialize)]
struct User {
    name: String,
    age: u32,
}

// #[deno_core::op2]
// async fn op_fetch(#[serde] url: String) -> Result<String, deno_core::error::AnyError> {
//     let response = reqwest::get(&url).await?;
//     let text = response.text().await?;
//     Ok(text)
// }

#[op2(fast)]
fn op_log(state: &mut OpState, #[string] message: String) -> Result<(), deno_error::JsErrorBox> {
    let handler = state.try_borrow::<Arc<String>>();

    // 自定义日志输出格式
    println!("[Rust Log] [{:?}] {}", handler, message);
    Ok(())
}

#[op2(async)]
async fn op_sum(
    state: Rc<RefCell<OpState>>, #[serde] nums: Vec<f64>,
) -> Result<f64, deno_error::JsErrorBox> {
    let state = state.borrow();
    let handler = state.try_borrow::<Arc<String>>();

    println!("handle: {:?}", handler);

    // Sum inputs
    let sum = nums.iter().fold(0.0, |a, v| a + v);
    // return as a Result<f64, OpError>
    Ok(sum)
}

pub async fn js_run() -> Result<String, deno_error::JsErrorBox> {
    const DECL: OpDecl = op_sum();
    const DECL_OP_LOG: OpDecl = op_log();
    const DECL_OP_FETCH: OpDecl = op_fetch();

    let ext = Extension {
        name: "my_ext",
        ops: std::borrow::Cow::Borrowed(&[DECL, DECL_OP_LOG, DECL_OP_FETCH]),
        middleware_fn: Some(Box::new(|op| {
            // println!("op: {:?}", op.name);
            match op.name {
                "op_print" => op.disable(),
                _ => op,
            }
        })),
        op_state_fn: Some(Box::new(|state| {
            let handler = Arc::new("hello".to_string());
            state.put(handler);
        })),
        ..Default::default()
    };

    let mut runtime =
        JsRuntime::new(RuntimeOptions { extensions: vec![ext], ..Default::default() });

    let user = User { name: "Bob".to_string(), age: 42 };
    {
        let scope = &mut runtime.handle_scope();

        // 使用 serde_v8 序列化 Rust 结构体到 JS 对象
        let key = v8::String::new(scope, "args").unwrap();
        let value = serde_v8::to_v8(scope, user).unwrap();

        let context = scope.get_current_context();
        let global = context.global(scope);
        global.set(scope, key.into(), value);
    }

    runtime
        .execute_script(
            "<SYSTEM>",
            r#"
            globalThis.fetch = async function(url, options = {}) {
                const config = {
                    url,
                    method: options.method || 'GET',
                    headers: options.headers,
                    body: options.body,
                    timeout: options.timeout,
                };
                
                try {
                    const response = await Deno.core.ops.op_fetch(config);
                    if (!response.ok) {
                        throw new Error(response.error || '请求失败');
                    }
                    return response;
                } catch (error) {
                    throw new Error(`请求异常: ${error.message}`);
                }
            };
            
            console.log = function(...args) {
                const message = args.map(arg => 
                    typeof arg === 'object' ? JSON.stringify(arg) : String(arg)
                ).join(' ');
                Deno.core.ops.op_log(message);
            };
        "#,
        )
        .unwrap();

    // 执行JavaScript
    runtime
        .execute_script(
            "define_async_function.js",
            r#"
            async function main(args, args2) {

                let ret = await Deno.core.ops.op_sum([1, 2, 3, 4, 5]);

                const ressponse = await fetch('https://www.baidu.com', {
                    timeout: 5000,
                    headers: {
                        'User-Agent': 'client',
                    },
                });
                console.log('GET Response:', ressponse);

                console.log('args', args);
                console.log('args', args2);
                return  ret;
            }

            // Deno.core.print("I'm broken");
            // console.log('args', `${args.age}`);
            // main(11)
            "#,
        )
        .unwrap();

    let result_val =
        runtime.execute_script("<execute>", r#"main(`${args.age}`, `${args.name}`)"#).unwrap();

    runtime.run_event_loop(Default::default()).await.unwrap();

    let scope = &mut runtime.handle_scope();

    if result_val.open(scope).is_promise() {
        let result_val = v8::Local::new(scope, result_val);
        let result_val = v8::Local::<v8::Promise>::try_from(result_val).unwrap();
        let result_val = result_val.result(scope);

        let result_val = serde_v8::from_v8::<serde_json::Value>(scope, result_val).unwrap();
        let result_val = serde_json::to_string(&result_val).unwrap();

        println!("async Result: {}", result_val);
        Ok(result_val)
    } else {
        let result_val = v8::Local::<v8::Value>::new(scope, result_val);
        let result_val = serde_v8::from_v8::<serde_json::Value>(scope, result_val).unwrap();
        let result_val = serde_json::to_string(&result_val).unwrap();
        println!("sync Result: {}", result_val);
        Ok(result_val)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test() {
        js_run().await.unwrap();
    }
}
