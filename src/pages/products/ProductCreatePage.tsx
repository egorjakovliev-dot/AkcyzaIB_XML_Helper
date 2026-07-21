import { useNavigate } from 'react-router-dom';
import { productsRepository } from '../../features/products/api/productsRepository';
import type { ProductInput } from '../../shared/api/client';
import { PageHeader } from '../../shared/components/ui';
import { ProductForm } from './ProductForm';
export function ProductCreatePage(){ const nav=useNavigate(); return <><PageHeader title="Add Product" description="Quick manual product creation for the local catalog."/><ProductForm onSubmit={async (input:ProductInput)=>{const p=await productsRepository.create(input); nav(`/products/${p.id}`);}}/></>; }
