export interface WowInstance {
  id: number;
  name: string;
  flavor_slug: string;
  root_path: string;
  addons_path: string;
  wtf_path: string | null;
  toc_suffixes: string[];
  is_default: boolean;
  created_at: string;
  updated_at: string;
}

export interface NewWowInstance {
  name: string;
  flavor_slug: string;
  root_path: string;
  addons_path: string;
  wtf_path: string | null;
  toc_suffixes: string[];
  is_default: boolean;
}

export interface SerializedAppError {
  code: string;
  message: string;
}