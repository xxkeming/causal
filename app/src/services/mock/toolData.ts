import { Tool, ToolCategory } from '../typings';

// 工具类别模拟数据
export const mockToolCategories: ToolCategory[] = [
  {
    id: 1,
    name: "网络工具"
  },
  {
    id: 2,
    name: "数据处理"
  },
  {
    id: 3,
    name: "开发工具"
  },
  {
    id: 4,
    name: "文档工具"
  },
  {
    id: 5,
    name: "媒体工具"
  }
];

// 工具模拟数据
export const mockTools: Tool[] = [
  {
    id: 1,
    categoryId: 1,
    name: "HTTP请求工具",
    description: "发送HTTP请求，支持GET、POST、PUT、DELETE等方法",
    iconId: 18, // 使用CompassOutline图标
    param: [
      {
        name: "url",
        type: "string",
        description: "请求地址URL",
        required: true,
        testValue: "https://api.example.com/data"
      },
      {
        name: "method",
        type: "string",
        description: "HTTP请求方法",
        required: true,
        testValue: "GET"
      },
      {
        name: "headers",
        type: "object",
        description: "请求头信息",
        required: false,
        testValue: '{"Content-Type": "application/json"}'
      },
      {
        name: "body",
        type: "string",
        description: "请求体内容",
        required: false,
        testValue: '{"query": "test"}'
      }
    ],
    code: `async function executeHttpRequest(params) {
  try {
    const options = {
      method: params.method,
      headers: params.headers ? JSON.parse(params.headers) : {},
    };
    
    if (params.body && ['POST', 'PUT', 'PATCH'].includes(params.method)) {
      options.body = params.body;
    }
    
    const response = await fetch(params.url, options);
    const data = await response.json();
    return {
      success: true,
      status: response.status,
      data: data
    };
  } catch (error) {
    return {
      success: false,
      error: error.message
    };
  }
}`,
    createdAt: "2023-09-01",
    updatedAt: "2023-09-15"
  },
  {
    id: 2,
    categoryId: 1,
    name: "网页爬取工具",
    description: "获取网页内容并提取文本和链接",
    iconId: 19, // 使用EarthOutline图标
    param: [
      {
        name: "url",
        type: "string",
        description: "网页地址",
        required: true,
        testValue: "https://example.com"
      },
      {
        name: "selector",
        type: "string",
        description: "CSS选择器，用于提取特定内容",
        required: false,
        testValue: ".main-content"
      }
    ],
    code: `async function scrapeWebpage(params) {
  try {
    const response = await fetch(params.url);
    const html = await response.text();
    
    // 简单的HTML解析，实际应用中可能需要更复杂的解析库
    const parser = new DOMParser();
    const doc = parser.parseFromString(html, 'text/html');
    
    // 提取内容
    let content;
    if (params.selector) {
      const element = doc.querySelector(params.selector);
      content = element ? element.textContent : null;
    } else {
      content = doc.body.textContent;
    }
    
    // 提取链接
    const links = Array.from(doc.querySelectorAll('a'))
      .map(a => ({
        text: a.textContent,
        href: a.getAttribute('href')
      }));
    
    return {
      success: true,
      content: content,
      links: links,
      title: doc.title
    };
  } catch (error) {
    return {
      success: false,
      error: error.message
    };
  }
}`,
    createdAt: "2023-09-02",
    updatedAt: "2023-09-16"
  },
  {
    id: 3,
    categoryId: 3,
    name: "代码执行工具",
    description: "安全执行JavaScript代码并返回结果",
    iconId: 23, // 使用FlashOutline图标
    param: [
      {
        name: "code",
        type: "string",
        description: "要执行的JavaScript代码",
        required: true,
        testValue: "return 2 + 2;"
      },
      {
        name: "timeout",
        type: "number",
        description: "代码执行超时时间(毫秒)",
        required: false,
        testValue: "3000"
      }
    ],
    code: `async function executeCode(params) {
  try {
    // 创建安全的执行环境
    const timeout = params.timeout || 5000;
    const code = params.code;
    
    // 使用Function构造函数创建可执行函数
    const executableFunction = new Function('console', \`
      let logs = [];
      const customConsole = {
        log: (...args) => {
          logs.push(args.map(arg => typeof arg === 'object' ? JSON.stringify(arg) : arg).join(' '));
        },
        error: (...args) => {
          logs.push('[ERROR] ' + args.map(arg => typeof arg === 'object' ? JSON.stringify(arg) : arg).join(' '));
        },
        warn: (...args) => {
          logs.push('[WARN] ' + args.map(arg => typeof arg === 'object' ? JSON.stringify(arg) : arg).join(' '));
        }
      };
      
      const result = (function() { 
        \${code} 
      })();
      
      return { result, logs };
    \`);
    
    // 添加超时机制
    const resultPromise = Promise.race([
      Promise.resolve(executableFunction(console)),
      new Promise((_, reject) => 
        setTimeout(() => reject(new Error(\`Code execution timed out after \${timeout}ms\`)), timeout)
      )
    ]);
    
    const { result, logs } = await resultPromise;
    
    return {
      success: true,
      result: result,
      logs: logs
    };
  } catch (error) {
    return {
      success: false,
      error: error.message
    };
  }
}`,
    createdAt: "2023-09-03",
    updatedAt: "2023-09-17"
  },
  {
    id: 4,
    categoryId: 4,
    name: "文件处理工具",
    description: "处理文本文件，支持读取和简单分析",
    iconId: 12, // 使用BuildOutline图标
    param: [
      {
        name: "content",
        type: "string",
        description: "文件内容",
        required: true,
        testValue: "这是一段测试文本\n可以包含多行内容\n用于测试文件处理工具"
      },
      {
        name: "operation",
        type: "string",
        description: "操作类型: analyze, count, extract",
        required: true,
        testValue: "analyze"
      },
      {
        name: "pattern",
        type: "string",
        description: "提取时使用的正则表达式",
        required: false,
        testValue: "\\d+"
      }
    ],
    code: `async function processFile(params) {
  try {
    const content = params.content;
    const operation = params.operation || 'analyze';
    const pattern = params.pattern || '';
    
    if (!content) {
      return {
        success: false,
        error: "Missing file content"
      };
    }
    
    switch(operation) {
      case 'analyze':
        const lines = content.split('\\n');
        const charCount = content.length;
        const wordCount = content.split(/\\s+/).filter(Boolean).length;
        
        return {
          success: true,
          analysis: {
            lines: lines.length,
            characters: charCount,
            words: wordCount,
            firstLine: lines[0],
            lastLine: lines[lines.length - 1]
          }
        };
        
      case 'count':
        return {
          success: true,
          counts: {
            lines: content.split('\\n').length,
            characters: content.length,
            words: content.split(/\\s+/).filter(Boolean).length
          }
        };
        
      case 'extract':
        if (!pattern) {
          return {
            success: false,
            error: "Pattern is required for extract operation"
          };
        }
        
        const regex = new RegExp(pattern, 'g');
        const matches = [...content.matchAll(regex)].map(match => match[0]);
        
        return {
          success: true,
          matches: matches
        };
        
      default:
        return {
          success: false,
          error: \`Unknown operation: \${operation}\`
        };
    }
  } catch (error) {
    return {
      success: false,
      error: error.message
    };
  }
}`,
    createdAt: "2023-09-04",
    updatedAt: "2023-09-18"
  },
  {
    id: 5,
    categoryId: 1,
    name: "API请求工具",
    description: "发送API请求并格式化结果",
    iconId: 31, // 使用LanguageOutline图标
    param: [
      {
        name: "endpoint",
        type: "string",
        description: "API终端点",
        required: true,
        testValue: "https://jsonplaceholder.typicode.com/posts/1"
      },
      {
        name: "method",
        type: "string",
        description: "请求方法",
        required: false,
        testValue: "GET"
      },
      {
        name: "format",
        type: "string",
        description: "响应格式(json, text, xml)",
        required: false,
        testValue: "json"
      }
    ],
    code: `async function makeAPIRequest(params) {
  try {
    const endpoint = params.endpoint;
    const method = params.method || 'GET';
    const format = params.format || 'json';
    
    const response = await fetch(endpoint, { method });
    
    let data;
    switch(format.toLowerCase()) {
      case 'json':
        data = await response.json();
        break;
      case 'text':
        data = await response.text();
        break;
      case 'xml':
        const text = await response.text();
        const parser = new DOMParser();
        data = parser.parseFromString(text, 'text/xml');
        // Convert XML to a more usable format for return
        const serializer = new XMLSerializer();
        data = serializer.serializeToString(data);
        break;
      default:
        data = await response.text();
    }
    
    return {
      success: true,
      status: response.status,
      headers: Object.fromEntries(response.headers.entries()),
      data: data
    };
  } catch (error) {
    return {
      success: false,
      error: error.message
    };
  }
}`,
    createdAt: "2023-09-05",
    updatedAt: "2023-09-19"
  },
  {
    id: 6,
    categoryId: 3,
    name: "Git操作工具",
    description: "执行Git命令并展示结果",
    iconId: 6, // 使用RocketOutline图标
    param: [
      {
        name: "command",
        type: "string",
        description: "Git命令(不包含'git'前缀)",
        required: true,
        testValue: "status"
      },
      {
        name: "repoPath",
        type: "string",
        description: "代码仓库路径",
        required: false,
        testValue: "./my-project"
      }
    ],
    code: `async function executeGitCommand(params) {
  try {
    // 注意：这在实际浏览器环境中不会工作，因为没有直接访问文件系统的权限
    // 此代码仅作为模拟示例
    
    const command = params.command || 'status';
    const repoPath = params.repoPath || '.';
    
    // 模拟Git命令执行
    // 在实际实现中，这可能是通过后端API调用来执行的
    console.log(\`Simulating 'git \${command}' in repository path: \${repoPath}\`);
    
    // 返回模拟结果
    return {
      success: true,
      command: \`git \${command}\`,
      output: \`Simulated output for 'git \${command}' in \${repoPath}\nThis is just mock data and would require server-side implementation.\`
    };
  } catch (error) {
    return {
      success: false,
      error: error.message
    };
  }
}`,
    createdAt: "2023-10-01",
    updatedAt: "2023-10-15"
  },
  {
    id: 7,
    categoryId: 2,
    name: "数学工具",
    description: "执行复杂的数学计算和公式求解",
    iconId: 33, // 使用MedalOutline图标
    param: [
      {
        name: "expression",
        type: "string",
        description: "数学表达式",
        required: true,
        testValue: "2 * (3 + 4)"
      },
      {
        name: "variables",
        type: "object",
        description: "表达式中使用的变量",
        required: false,
        testValue: '{"x": 5, "y": 3}'
      },
      {
        name: "operation",
        type: "string",
        description: "操作类型: evaluate, simplify, solve",
        required: false,
        testValue: "evaluate"
      }
    ],
    code: `async function mathCalculator(params) {
  try {
    const expression = params.expression;
    const operation = params.operation || 'evaluate';
    let variables = {};
    
    try {
      if (params.variables) {
        variables = typeof params.variables === 'string' 
          ? JSON.parse(params.variables) 
          : params.variables;
      }
    } catch (e) {
      return {
        success: false,
        error: \`Invalid variables format: \${e.message}\`
      };
    }
    
    // 简单的表达式求值函数
    const evaluateExpression = (expr, vars) => {
      // 替换变量
      let evalExpr = expr;
      for (const [key, value] of Object.entries(vars)) {
        const regex = new RegExp(\`\\\\b\${key}\\\\b\`, 'g');
        evalExpr = evalExpr.replace(regex, value);
      }
      
      // 安全的eval，仅用于简单计算
      // 实际应用中应使用专业的数学库
      return Function('return ' + evalExpr)();
    };
    
    switch(operation) {
      case 'evaluate':
        return {
          success: true,
          result: evaluateExpression(expression, variables),
          expression: expression
        };
        
      case 'simplify':
        // 简单的简化示例，实际应用需要数学库
        return {
          success: true,
          result: \`Simplified: \${expression}\`,
          note: "This is a mock implementation. Actual simplification requires a math library."
        };
        
      case 'solve':
        // 求解示例，实际应用需要数学库
        return {
          success: true,
          result: \`Solution for \${expression}\`,
          note: "This is a mock implementation. Actual equation solving requires a math library."
        };
        
      default:
        return {
          success: false,
          error: \`Unknown operation: \${operation}\`
        };
    }
  } catch (error) {
    return {
      success: false,
      error: error.message
    };
  }
}`,
    createdAt: "2023-10-02",
    updatedAt: "2023-10-16"
  },
  {
    id: 8,
    categoryId: 2,
    name: "数据分析工具",
    description: "分析数值数据并生成统计信息",
    iconId: 2, // 使用HeartOutline图标
    param: [
      {
        name: "data",
        type: "string",
        description: "数值数据，以逗号分隔",
        required: true,
        testValue: "5,12,23,17,9,14,6,8,20,15"
      },
      {
        name: "operations",
        type: "string",
        description: "统计操作：basic, full, correlation",
        required: false,
        testValue: "basic"
      },
      {
        name: "secondData",
        type: "string",
        description: "用于相关性分析的第二组数据",
        required: false,
        testValue: "8,15,20,12,7,10,5,9,18,14"
      }
    ],
    code: `async function analyzeData(params) {
  try {
    // 解析数据
    const dataStr = params.data;
    const operations = params.operations || 'basic';
    const secondDataStr = params.secondData || '';
    
    if (!dataStr) {
      return {
        success: false,
        error: "No data provided"
      };
    }
    
    const data = dataStr.split(',').map(Number).filter(n => !isNaN(n));
    
    if (data.length === 0) {
      return {
        success: false,
        error: "No valid numeric data found"
      };
    }
    
    // 基础统计函数
    const calculateBasicStats = (arr) => {
      const sum = arr.reduce((a, b) => a + b, 0);
      const mean = sum / arr.length;
      const min = Math.min(...arr);
      const max = Math.max(...arr);
      const range = max - min;
      
      return { sum, mean, min, max, range };
    };
    
    // 计算标准差
    const calculateStdDev = (arr, mean) => {
      const squareDiffs = arr.map(value => {
        const diff = value - mean;
        return diff * diff;
      });
      const avgSquareDiff = squareDiffs.reduce((a, b) => a + b, 0) / arr.length;
      return Math.sqrt(avgSquareDiff);
    };
    
    // 计算中位数
    const calculateMedian = (arr) => {
      const sorted = [...arr].sort((a, b) => a - b);
      const middle = Math.floor(sorted.length / 2);
      
      if (sorted.length % 2 === 0) {
        return (sorted[middle - 1] + sorted[middle]) / 2;
      } else {
        return sorted[middle];
      }
    };
    
    // 计算两组数据的相关性
    const calculateCorrelation = (arrX, arrY) => {
      if (arrX.length !== arrY.length) {
        throw new Error("Data sets must be of equal length for correlation analysis");
      }
      
      const n = arrX.length;
      const meanX = arrX.reduce((a, b) => a + b, 0) / n;
      const meanY = arrY.reduce((a, b) => a + b, 0) / n;
      
      let numerator = 0;
      let denomX = 0;
      let denomY = 0;
      
      for (let i = 0; i < n; i++) {
        const diffX = arrX[i] - meanX;
        const diffY = arrY[i] - meanY;
        numerator += diffX * diffY;
        denomX += diffX * diffX;
        denomY += diffY * diffY;
      }
      
      return numerator / Math.sqrt(denomX * denomY);
    };
    
    let result = {};
    
    // 执行请求的操作
    if (operations === 'basic' || operations === 'full') {
      const basicStats = calculateBasicStats(data);
      result = { ...result, ...basicStats };
      
      // 添加数据大小
      result.count = data.length;
    }
    
    if (operations === 'full') {
      result.stdDev = calculateStdDev(data, result.mean);
      result.median = calculateMedian(data);
    }
    
    if (operations === 'correlation') {
      if (!secondDataStr) {
        return {
          success: false,
          error: "Second data set required for correlation analysis"
        };
      }
      
      const secondData = secondDataStr.split(',').map(Number).filter(n => !isNaN(n));
      
      if (secondData.length === 0) {
        return {
          success: false,
          error: "No valid numeric data found in second data set"
        };
      }
      
      if (data.length !== secondData.length) {
        return {
          success: false,
          error: \`Data sets have different lengths: \${data.length} vs \${secondData.length}\`
        };
      }
      
      result.correlation = calculateCorrelation(data, secondData);
    }
    
    return {
      success: true,
      data: data,
      stats: result
    };
  } catch (error) {
    return {
      success: false,
      error: error.message
    };
  }
}`,
    createdAt: "2023-10-03",
    updatedAt: "2023-10-17"
  },
  {
    id: 9,
    categoryId: 5,
    name: "数据可视化工具",
    description: "生成数据可视化描述，可用于图表生成",
    iconId: 16, // 使用ColorPaletteOutline图标
    param: [
      {
        name: "data",
        type: "string",
        description: "要可视化的数据，JSON格式",
        required: true,
        testValue: '[{"month":"一月","sales":120},{"month":"二月","sales":140},{"month":"三月","sales":180}]'
      },
      {
        name: "chartType",
        type: "string",
        description: "图表类型: bar, line, pie, scatter",
        required: true,
        testValue: "line"
      },
      {
        name: "title",
        type: "string",
        description: "图表标题",
        required: false,
        testValue: "月度销售额"
      }
    ],
    code: `async function visualizeData(params) {
  try {
    const chartType = params.chartType || 'bar';
    const title = params.title || 'Data Visualization';
    let data;
    
    // 解析数据
    try {
      data = typeof params.data === 'string' ? JSON.parse(params.data) : params.data;
    } catch (e) {
      return {
        success: false,
        error: \`Invalid data format: \${e.message}\`
      };
    }
    
    if (!Array.isArray(data)) {
      return {
        success: false,
        error: "Data must be an array"
      };
    }
    
    // 生成图表配置
    const generateChartConfig = () => {
      // 根据数据分析键
      const keys = Object.keys(data[0] || {});
      const config = {
        type: chartType,
        title: title,
        data: data
      };
      
      // 添加一些图表类型特定的建议
      switch(chartType) {
        case 'bar':
          if (keys.length >= 2) {
            config.xAxis = keys[0];
            config.yAxis = keys[1];
          }
          break;
          
        case 'line':
          if (keys.length >= 2) {
            config.xAxis = keys[0];
            config.yAxis = keys[1];
            config.showPoints = true;
          }
          break;
          
        case 'pie':
          if (keys.length >= 2) {
            config.labelField = keys[0];
            config.valueField = keys[1];
          }
          break;
          
        case 'scatter':
          if (keys.length >= 3) {
            config.xField = keys[0];
            config.yField = keys[1];
            config.sizeField = keys[2];
          } else if (keys.length >= 2) {
            config.xField = keys[0];
            config.yField = keys[1];
          }
          break;
      }
      
      return config;
    };
    
    // 生成图表配置
    const chartConfig = generateChartConfig();
    
    // 生成图表描述
    const generateChartDescription = (config) => {
      const type = config.type;
      const title = config.title;
      
      switch(type) {
        case 'bar':
          return \`柱状图展示了 "\${title}" 的数据分布，X轴是 \${config.xAxis}，Y轴是 \${config.yAxis}。\`;
          
        case 'line':
          return \`折线图展示了 "\${title}" 的趋势，X轴是 \${config.xAxis}，Y轴是 \${config.yAxis}。\`;
          
        case 'pie':
          return \`饼图展示了 "\${title}" 的比例分布，标签是 \${config.labelField}，数值是 \${config.valueField}。\`;
          
        case 'scatter':
          return \`散点图展示了 "\${title}" 的数据关系，X轴是 \${config.xField}，Y轴是 \${config.yField}。\`;
          
        default:
          return \`图表展示了 "\${title}" 的数据。\`;
      }
    };
    
    return {
      success: true,
      chartConfig: chartConfig,
      description: generateChartDescription(chartConfig),
      dataPoints: data.length
    };
  } catch (error) {
    return {
      success: false,
      error: error.message
    };
  }
}`,
    createdAt: "2023-10-04",
    updatedAt: "2023-10-18"
  }
];
