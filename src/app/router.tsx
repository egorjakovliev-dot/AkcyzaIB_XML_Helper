import { createBrowserRouter } from 'react-router-dom';
import { AppLayout } from './layouts/AppLayout';
import { DashboardPage } from '../pages/dashboard/DashboardPage';
import { PlaceholderPage } from '../pages/PlaceholderPage';
import { ProductsPage } from '../pages/products/ProductsPage';
import { ProductCreatePage } from '../pages/products/ProductCreatePage';
import { ProductDetailsPage } from '../pages/products/ProductDetailsPage';
import { CompetitorsPage } from '../pages/competitors/CompetitorsPage';
import { CompetitorCreatePage } from '../pages/competitors/CompetitorCreatePage';
import { SettingsPage } from '../pages/settings/SettingsPage';
export const router = createBrowserRouter([{ path: '/', element: <AppLayout />, children: [
  { index: true, element: <DashboardPage /> }, { path: 'ai-advisor', element: <PlaceholderPage title="AI Advisor" /> }, { path: 'products', element: <ProductsPage /> }, { path: 'products/new', element: <ProductCreatePage /> }, { path: 'products/:productId', element: <ProductDetailsPage /> }, { path: 'competitors', element: <CompetitorsPage /> }, { path: 'competitors/new', element: <CompetitorCreatePage /> }, { path: 'seo', element: <PlaceholderPage title="SEO" /> }, { path: 'market', element: <PlaceholderPage title="Market" /> }, { path: 'sales', element: <PlaceholderPage title="Sales" /> }, { path: 'finance', element: <PlaceholderPage title="Finance" /> }, { path: 'purchases', element: <PlaceholderPage title="Purchases" /> }, { path: 'recommendations', element: <PlaceholderPage title="Recommendations" /> }, { path: 'tasks', element: <PlaceholderPage title="Tasks" /> }, { path: 'imports', element: <PlaceholderPage title="Imports" /> }, { path: 'exports', element: <PlaceholderPage title="Exports" /> }, { path: 'settings', element: <SettingsPage /> }
]}]);
