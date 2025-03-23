import { defineStore } from 'pinia';
import { ref } from 'vue';
import { markRaw } from 'vue'; // 导入markRaw函数
import { 
  PersonOutline, HeartOutline, StarOutline, DiamondOutline,
  AirplaneOutline, RocketOutline, LeafOutline, FlowerOutline,
  PawOutline, BalloonOutline, BicycleOutline, BuildOutline,
  BusOutline, CafeOutline, CameraOutline, CarOutline,
  ColorPaletteOutline, CompassOutline, EarthOutline, EyeOutline,
  FastFoodOutline, FlagOutline, FlashOutline, GameControllerOutline,
  GiftOutline, GlobeOutline, HappyOutline, HeadsetOutline,
  HomeOutline, IceCreamOutline, LanguageOutline, LaptopOutline,
  MedalOutline, MoonOutline, MusicalNoteOutline, PlanetOutline,
  PrismOutline, SchoolOutline, ShapesOutline, SparklesOutline,
  UmbrellaOutline, WineOutline
} from '@vicons/ionicons5';

// 定义头像图标选项接口
export interface IconOption {
  id: number;
  icon: any;
  name: string;
  color: string;
}

// 预设的颜色映射 - 为每个图标分配一个固定的颜色
const iconColorMap = {
  PersonOutline: '#2080f0',
  HeartOutline: '#d03050',
  StarOutline: '#f0a020',
  DiamondOutline: '#8a2be2',
  AirplaneOutline: '#1e90ff',
  RocketOutline: '#ff6347',
  LeafOutline: '#18a058',
  FlowerOutline: '#ff69b4',
  PawOutline: '#7b68ee',
  BalloonOutline: '#ff8c00',
  BicycleOutline: '#3cb371',
  BuildOutline: '#0C7A4A',
  BusOutline: '#9370db',
  CafeOutline: '#20b2aa',
  CameraOutline: '#ff4500',
  CarOutline: '#2080f0',
  ColorPaletteOutline: '#f0a020',
  CompassOutline: '#8a2be2',
  EarthOutline: '#1e90ff',
  EyeOutline: '#d03050',
  FastFoodOutline: '#18a058',
  FlagOutline: '#ff6347',
  FlashOutline: '#ff69b4',
  GameControllerOutline: '#7b68ee',
  GiftOutline: '#ff8c00',
  GlobeOutline: '#3cb371',
  HappyOutline: '#0C7A4A',
  HeadsetOutline: '#9370db',
  HomeOutline: '#20b2aa',
  IceCreamOutline: '#ff4500',
  LanguageOutline: '#2080f0',
  LaptopOutline: '#d03050',
  MedalOutline: '#f0a020',
  MoonOutline: '#8a2be2',
  MusicalNoteOutline: '#1e90ff',
  PlanetOutline: '#ff6347',
  PrismOutline: '#18a058',
  SchoolOutline: '#ff69b4',
  ShapesOutline: '#7b68ee',
  SparklesOutline: '#ff8c00',
  UmbrellaOutline: '#3cb371',
  WineOutline: '#0C7A4A'
};

export const useIconStore = defineStore('icon', () => {
  // 所有图标组件的映射 - 使用markRaw处理每个图标组件
  const iconComponents = {
    PersonOutline: markRaw(PersonOutline), 
    HeartOutline: markRaw(HeartOutline), 
    StarOutline: markRaw(StarOutline), 
    DiamondOutline: markRaw(DiamondOutline),
    AirplaneOutline: markRaw(AirplaneOutline), 
    RocketOutline: markRaw(RocketOutline), 
    LeafOutline: markRaw(LeafOutline), 
    FlowerOutline: markRaw(FlowerOutline),
    PawOutline: markRaw(PawOutline), 
    BalloonOutline: markRaw(BalloonOutline), 
    BicycleOutline: markRaw(BicycleOutline), 
    BuildOutline: markRaw(BuildOutline),
    BusOutline: markRaw(BusOutline), 
    CafeOutline: markRaw(CafeOutline), 
    CameraOutline: markRaw(CameraOutline), 
    CarOutline: markRaw(CarOutline),
    ColorPaletteOutline: markRaw(ColorPaletteOutline), 
    CompassOutline: markRaw(CompassOutline), 
    EarthOutline: markRaw(EarthOutline), 
    EyeOutline: markRaw(EyeOutline),
    FastFoodOutline: markRaw(FastFoodOutline), 
    FlagOutline: markRaw(FlagOutline), 
    FlashOutline: markRaw(FlashOutline), 
    GameControllerOutline: markRaw(GameControllerOutline),
    GiftOutline: markRaw(GiftOutline), 
    GlobeOutline: markRaw(GlobeOutline), 
    HappyOutline: markRaw(HappyOutline), 
    HeadsetOutline: markRaw(HeadsetOutline),
    HomeOutline: markRaw(HomeOutline), 
    IceCreamOutline: markRaw(IceCreamOutline), 
    LanguageOutline: markRaw(LanguageOutline), 
    LaptopOutline: markRaw(LaptopOutline),
    MedalOutline: markRaw(MedalOutline), 
    MoonOutline: markRaw(MoonOutline), 
    MusicalNoteOutline: markRaw(MusicalNoteOutline), 
    PlanetOutline: markRaw(PlanetOutline),
    PrismOutline: markRaw(PrismOutline), 
    SchoolOutline: markRaw(SchoolOutline), 
    ShapesOutline: markRaw(ShapesOutline), 
    SparklesOutline: markRaw(SparklesOutline),
    UmbrellaOutline: markRaw(UmbrellaOutline), 
    WineOutline: markRaw(WineOutline)
  };

  // 生成所有图标选项，使用固定的颜色
  const generateIcons = (): IconOption[] => {
    const result: IconOption[] = [];
    const iconNames = Object.keys(iconComponents);
    
    // 为每个图标创建一个选项，使用预定义的颜色
    for (let i = 0; i < iconNames.length; i++) {
      const iconName = iconNames[i];
      const formattedName = iconName
        .replace('Outline', '')
        .replace(/([A-Z])/g, ' $1')
        .trim();
      
      result.push({
        id: i + 1,
        icon: iconComponents[iconName as keyof typeof iconComponents],
        name: formattedName,
        color: iconColorMap[iconName as keyof typeof iconColorMap] || '#18a058'  // 使用固定颜色，如果没有映射则使用默认颜色
      });
    }
    
    return result;
  };

  // 图标列表 - 只生成一次
  const icons = ref<IconOption[]>(generateIcons());

  // 通过ID获取图标
  const getIconById = (id: number): IconOption | undefined => {
    return icons.value.find(icon => icon.id === id);
  };

  // 通过名称搜索图标
  const searchIcons = (keyword: string): IconOption[] => {
    if (!keyword) return icons.value;
    const lowerKeyword = keyword.toLowerCase();
    return icons.value.filter(icon => 
      icon.name.toLowerCase().includes(lowerKeyword)
    );
  };

  // 为单个图标设置颜色 - 仅用于自定义颜色的情况
  const setIconColor = (id: number, color: string) => {
    const icon = icons.value.find(i => i.id === id);
    if (icon) {
      icon.color = color;
    }
  };
  
  return {
    icons,
    getIconById,
    searchIcons,
    setIconColor
  };
});
