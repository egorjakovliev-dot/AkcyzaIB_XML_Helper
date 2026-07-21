import { invoke } from '@tauri-apps/api/core';

export type Product = { id:number; sku:string; name:string; allegro_offer_id?:string|null; ean?:string|null; category:string; brand?:string|null; status:string; purchase_price:number; selling_price:number; currency:string; stock_quantity:number; min_stock_quantity:number; url?:string|null; notes?:string|null; created_at:string; updated_at:string };
export type ProductInput = Omit<Product, 'id'|'created_at'|'updated_at'>;
export type Competitor = { id:number; name:string; marketplace:string; url:string; rating?:number|null; review_count?:number|null; notes?:string|null; created_at:string; updated_at:string };
export type CompetitorInput = Omit<Competitor, 'id'|'created_at'|'updated_at'>;
export type SettingsPayload = { store_name:string; currency:string; language:string; theme:string };

export const api = {
  listProducts: () => invoke<Product[]>('list_products'),
  getProduct: (id:number) => invoke<Product | null>('get_product', { id }),
  createProduct: (input: ProductInput) => invoke<Product>('create_product', { input }),
  updateProduct: (id:number, input: ProductInput) => invoke<Product>('update_product', { id, input }),
  listCompetitors: () => invoke<Competitor[]>('list_competitors'),
  createCompetitor: (input: CompetitorInput) => invoke<Competitor>('create_competitor', { input }),
  getSettings: () => invoke<SettingsPayload>('get_settings'),
  saveSettings: (input: SettingsPayload) => invoke<SettingsPayload>('save_settings', { input })
};
