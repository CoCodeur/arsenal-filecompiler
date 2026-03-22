import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export interface NasConfig {
  server: string;
  share_path: string;
  username: string;
  password: string;
}

export interface Product {
  category: string;
  reference: string;
  description: string;
  material: string;
  quantity: number;
}

export interface Order {
  id: string;
  client: string;
  address: string;
  date: string;
  products: Product[];
}

export interface CompilationProgress {
  current: number;
  total: number;
  message: string;
  status: "processing" | "success" | "error";
}

export interface CompilationResult {
  success: boolean;
  files_copied: number;
  files_missing: string[];
  errors: string[];
  output_path: string;
}

export interface AppConfig {
  nas: NasConfig;
  default_output_dir: string;
}

/** Parse an XML order file and return structured data */
export async function parseXmlFile(path: string): Promise<Order> {
  return invoke("parse_xml_file", { path });
}

/** Run the full compilation: create folders + copy PDFs */
export async function compileOrder(
  xmlPath: string,
  outputDir: string,
  nasConfig: NasConfig
): Promise<CompilationResult> {
  return invoke("compile_order", { xmlPath, outputDir, nasConfig });
}

/** Load saved configuration */
export async function loadConfig(): Promise<AppConfig> {
  return invoke("load_config");
}

/** Save configuration */
export async function saveConfig(config: AppConfig): Promise<void> {
  return invoke("save_config", { config });
}

/** Test NAS connection */
export async function testNasConnection(
  config: NasConfig
): Promise<{ success: boolean; message: string }> {
  return invoke("test_nas_connection", { nasConfig: config });
}

/** Listen for compilation progress events */
export function onProgress(
  callback: (progress: CompilationProgress) => void
): Promise<() => void> {
  return listen<CompilationProgress>("compilation-progress", (event) => {
    callback(event.payload);
  });
}
