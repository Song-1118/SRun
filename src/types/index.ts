export interface CpuInfo {
  name: string;
  cores: number;
  threads: number;
  base_frequency: number;
  max_frequency: number;
  cache_size: number;
}

export interface GpuInfo {
  name: string;
  vendor: string;
  memory: number;
  driver_version: string | null;
}

export interface MemoryInfo {
  total: number;
  available: number;
  used: number;
}

export type DiskType = 'ssd' | 'hdd' | 'unknown';

export interface StorageInfo {
  name: string;
  device: string;
  total: number;
  available: number;
  disk_type: DiskType;
}

export interface OsInfo {
  name: string;
  version: string;
  build: string;
  architecture: string;
}

export interface SystemInfo {
  cpu: CpuInfo;
  gpu: GpuInfo[];
  memory: MemoryInfo;
  storage: StorageInfo[];
  os: OsInfo;
  detected_at: string;
}

export interface GameInfo {
  id: string;
  name: string;
  cover_url: string;
  developer: string;
  publisher: string;
  release_date: string | null;
  genres: string[];
}

export interface Requirement {
  cpu: string;
  cpu_core_count: number | null;
  cpu_frequency: number | null;
  gpu: string;
  gpu_memory: number | null;
  memory: number;
  storage: number;
  os: string;
}

export interface GameRequirements {
  game_id: string;
  minimum: Requirement;
  recommended: Requirement;
}

export type MatchLevel = 'perfect' | 'smooth' | 'basic' | 'impossible';

export interface ComponentResult {
  name: string;
  user_value: string;
  required_value: string;
  meets_minimum: boolean;
  meets_recommended: boolean;
  benchmark_score: number | null;
  required_score: number | null;
  percentage_above_minimum: number;
  percentage_above_recommended: number;
}

export type BenchmarkType = 'cpu' | 'gpu';

export interface BenchmarkData {
  id: string;
  name: string;
  brand: string;
  type: BenchmarkType;
  score: number;
  aliases: string[];
}

export type SuggestionType = 'game_setting' | 'hardware_upgrade' | 'system_optimization';

export type Priority = 'high' | 'medium' | 'low';

export interface Suggestion {
  id: string;
  type: SuggestionType;
  title: string;
  content: string;
  priority: Priority;
}

export interface ComparisonResult {
  game_id: string;
  game_name: string;
  score: number;
  level: MatchLevel;
  cpu_result: ComponentResult;
  gpu_result: ComponentResult;
  memory_result: ComponentResult;
  storage_result: ComponentResult;
  suggestions: Suggestion[];
  detected_at: string;
}

export interface HistoryRecord {
  id: string;
  game_id: string;
  game_name: string;
  score: number;
  level: MatchLevel;
  detected_at: string;
}

export interface GameListResponse {
  games: GameInfo[];
  total: number;
  page: number;
  page_size: number;
}

export interface Settings {
  theme: string;
  language: string;
  auto_update: boolean;
  data_update_frequency: number;
}

export interface DetectionProgress {
  step: string;
  progress: number;
}

export interface DataUpdateInfo {
  version: string;
}
