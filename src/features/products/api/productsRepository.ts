import { api, type ProductInput } from '../../../shared/api/client';
export const productsRepository = { list: api.listProducts, get: api.getProduct, create: (input: ProductInput) => api.createProduct(input), update: (id:number, input: ProductInput) => api.updateProduct(id, input) };
