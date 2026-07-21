# Архитектура первой версии AkcyzaIB XML Helper

## 1. Назначение первой версии

Первая версия продукта — это interactive prototype персональной desktop-first программы для управления магазином на Allegro. Прототип должен помочь проверить удобство интерфейса, структуру разделов, навигацию, сценарии работы с товарами и конкурентами, а также подготовить основу для будущей аналитики и AI-рекомендаций.

Продукт не является SaaS и не проектируется как коммерческая многопользовательская система. Основной пользовательский сценарий — ежедневное открытие приложения владельцем магазина для быстрого понимания состояния бизнеса и принятия решений, повышающих прибыль.

## 2. Ключевые архитектурные принципы

- Desktop-first: приложение должно удобно работать на Windows как установленная программа.
- PWA-ready: фронтенд должен быть спроектирован так, чтобы в будущем его можно было использовать как PWA без переписывания UI.
- Local-first: первая версия хранит данные локально, быстро открывается и не зависит от внешних сервисов.
- Простая навигация: все основные разделы доступны из левого меню.
- Минимум действий пользователя: добавление товара, конкурента, поиск и фильтры должны быть быстрыми.
- Модульность: каждый бизнес-раздел оформляется как отдельный feature-модуль.
- Заглушки разрешены: разделы без реализованной логики должны иметь качественные placeholder-экраны.
- Без преждевременной сложности: AI, парсинг, прогнозы и автоматические рекомендации не входят в первую версию.

## 3. Предлагаемый стек технологий

### 3.1 Desktop shell

Рекомендуемый вариант: **Tauri 2 + Rust shell**.

Причины выбора:

- малый размер установочного файла по сравнению с Electron;
- хорошая производительность;
- нативная работа на Windows;
- возможность использовать современный веб-фронтенд;
- проще сохранить путь к будущей PWA, потому что UI остается web-based.

Альтернатива: Electron. Его стоит выбирать только если потребуется очень богатая Node.js-экосистема внутри desktop-оболочки. Для текущей персональной программы Tauri предпочтительнее.

### 3.2 Frontend

Рекомендуемый стек:

- React;
- TypeScript;
- Vite;
- React Router;
- TanStack Query для слоя запросов и кэширования;
- Zustand для простого UI/application state;
- React Hook Form + Zod для форм и валидации;
- Tailwind CSS;
- shadcn/ui или собственный набор компонентов на Radix UI;
- Lucide Icons;
- Recharts для будущих простых графиков.

### 3.3 Локальная база данных

Рекомендуемый вариант для первой версии: **SQLite**.

Варианты доступа:

- через Tauri plugin/sql или Rust-команды;
- с миграциями через sqlx/refinery на стороне Tauri;
- в будущем можно вынести слой данных в отдельный backend/API без переписывания UI.

Почему SQLite:

- локально;
- быстро;
- надежно;
- не требует отдельного сервера;
- достаточно для одного пользователя;
- легко делать backup/export.

### 3.4 Тестирование и качество

Минимальный набор для первой версии:

- ESLint;
- Prettier;
- TypeScript strict mode;
- Vitest для unit-тестов утилит и логики;
- Playwright для smoke-тестов ключевой навигации;
- миграции базы данных, хранящиеся в репозитории.

## 4. Общая архитектура проекта

Архитектура должна быть разделена на пять уровней:

1. **App shell** — инициализация приложения, роутинг, layout, providers.
2. **Pages** — экраны, соответствующие маршрутам.
3. **Features** — бизнес-модули: products, competitors, settings, imports и другие.
4. **Shared** — переиспользуемые UI-компоненты, утилиты, хуки, типы.
5. **Data access** — единый слой доступа к локальной базе данных.

Главное правило: страницы не должны напрямую знать детали SQLite/Tauri. Они работают с feature-сервисами или repository-адаптерами.

## 5. Предлагаемая структура папок

```text
AkcyzaIB_XML_Helper/
  docs/
    ARCHITECTURE.md
  src-tauri/
    capabilities/
    migrations/
      0001_initial.sql
    src/
      commands/
        products.rs
        competitors.rs
        settings.rs
      db/
        mod.rs
        migrations.rs
      models/
      main.rs
    tauri.conf.json
  src/
    app/
      App.tsx
      router.tsx
      providers.tsx
      layouts/
        AppLayout.tsx
        Sidebar.tsx
        TopBar.tsx
    pages/
      dashboard/
        DashboardPage.tsx
      ai-advisor/
        AiAdvisorPage.tsx
      products/
        ProductsPage.tsx
        ProductDetailsPage.tsx
        ProductCreatePage.tsx
      competitors/
        CompetitorsPage.tsx
        CompetitorCreatePage.tsx
      seo/
        SeoPage.tsx
      market/
        MarketPage.tsx
      sales/
        SalesPage.tsx
      finance/
        FinancePage.tsx
      purchases/
        PurchasesPage.tsx
      recommendations/
        RecommendationsPage.tsx
      tasks/
        TasksPage.tsx
      imports/
        ImportsPage.tsx
      exports/
        ExportsPage.tsx
      settings/
        SettingsPage.tsx
    features/
      products/
        api/
          productsRepository.ts
        components/
          ProductCard.tsx
          ProductFilters.tsx
          ProductForm.tsx
          ProductsTable.tsx
        hooks/
          useProducts.ts
        schemas/
          productSchema.ts
        types.ts
      competitors/
        api/
          competitorsRepository.ts
        components/
          CompetitorForm.tsx
          CompetitorsTable.tsx
        hooks/
          useCompetitors.ts
        schemas/
          competitorSchema.ts
        types.ts
      settings/
        api/
        components/
        types.ts
    shared/
      api/
        client.ts
        result.ts
      components/
        ui/
        AppPageHeader.tsx
        EmptyState.tsx
        FilterBar.tsx
        SearchInput.tsx
        StatCard.tsx
        DataTable.tsx
        StatusBadge.tsx
        ConfirmDialog.tsx
      constants/
        navigation.ts
      hooks/
      lib/
        cn.ts
        formatters.ts
      types/
    styles/
      globals.css
    main.tsx
  package.json
  vite.config.ts
  tsconfig.json
```

## 6. Структура базы данных

Первая версия должна хранить только данные, нужные для прототипа: товары, конкурентов, настройки и базовые связи. Схему лучше сразу проектировать так, чтобы позже добавить импорт, продажи, финансы и рекомендации.

### 6.1 Таблица `products`

Назначение: локальный каталог товаров магазина.

Поля:

- `id` — primary key;
- `sku` — внутренний артикул;
- `name` — название товара;
- `allegro_offer_id` — ID оффера Allegro, nullable;
- `ean` — EAN/GTIN, nullable;
- `category` — категория;
- `brand` — бренд, nullable;
- `status` — draft, active, paused, archived;
- `purchase_price` — закупочная цена;
- `selling_price` — текущая цена продажи;
- `currency` — валюта, по умолчанию PLN;
- `stock_quantity` — остаток;
- `min_stock_quantity` — минимальный желаемый остаток;
- `url` — ссылка на оффер, nullable;
- `notes` — заметки;
- `created_at`;
- `updated_at`.

### 6.2 Таблица `competitors`

Назначение: каталог конкурентов и наблюдаемых предложений.

Поля:

- `id` — primary key;
- `name` — имя конкурента или магазина;
- `marketplace` — например Allegro;
- `url` — ссылка на магазин или оффер;
- `rating` — рейтинг, nullable;
- `review_count` — количество отзывов, nullable;
- `notes` — заметки;
- `created_at`;
- `updated_at`.

### 6.3 Таблица `competitor_offers`

Назначение: связь конкурентов с конкретными товарами пользователя.

Поля:

- `id` — primary key;
- `product_id` — foreign key на `products.id`;
- `competitor_id` — foreign key на `competitors.id`;
- `title` — название оффера конкурента;
- `url` — ссылка на оффер;
- `price` — цена;
- `currency` — валюта;
- `delivery_price` — стоимость доставки, nullable;
- `availability` — наличие;
- `last_checked_at` — дата ручной или будущей автоматической проверки;
- `created_at`;
- `updated_at`.

### 6.4 Таблица `settings`

Назначение: хранение пользовательских настроек.

Поля:

- `key` — primary key;
- `value` — JSON/text;
- `updated_at`.

### 6.5 Таблица `tasks`

Назначение: простые ручные задачи пользователя.

Поля:

- `id` — primary key;
- `title`;
- `description`;
- `status` — open, in_progress, done, archived;
- `priority` — low, medium, high;
- `related_entity_type` — product, competitor, finance, import, general;
- `related_entity_id` — nullable;
- `due_date` — nullable;
- `created_at`;
- `updated_at`.

### 6.6 Таблица `app_events`

Назначение: простой журнал действий для будущей аналитики и диагностики.

Поля:

- `id` — primary key;
- `event_type`;
- `entity_type`;
- `entity_id`;
- `payload` — JSON/text;
- `created_at`.

## 7. Навигация между экранами

Основная навигация находится в левом sidebar. Sidebar всегда видим на desktop.

Маршруты первой версии:

```text
/                       -> Dashboard
/ai-advisor              -> AI Advisor placeholder
/products                -> Products catalog
/products/new            -> Add product
/products/:productId     -> Product details
/competitors             -> Competitors catalog
/competitors/new         -> Add competitor
/seo                     -> SEO placeholder
/market                  -> Market placeholder
/sales                   -> Sales placeholder
/finance                 -> Finance placeholder
/purchases               -> Purchases placeholder
/recommendations         -> Recommendations placeholder
/tasks                   -> Tasks placeholder or simple task list
/imports                 -> Imports placeholder
/exports                 -> Exports placeholder
/settings                -> Settings
```

Переходы:

- Dashboard -> Products через карточку количества товаров.
- Dashboard -> Competitors через карточку конкурентов.
- Products -> Product Details по клику на строку/карточку товара.
- Products -> Add Product через primary action.
- Product Details -> Products через back action.
- Competitors -> Add Competitor через primary action.
- Settings доступен всегда из sidebar.

## 8. Полный список экранов первой версии

### 8.1 Dashboard

Цель: дать быстрый обзор бизнеса и направить пользователя к действиям.

Содержимое:

- приветственный блок;
- карточки: товары, активные товары, конкуренты, задачи;
- блок Today Focus с ручными подсказками-заглушками;
- быстрые действия: добавить товар, добавить конкурента, открыть импорт;
- placeholder будущего AI-бизнес-консультанта.

### 8.2 AI Advisor

Цель: показать будущее место AI-консультанта.

Содержимое:

- красивый empty/coming soon state;
- объяснение, что в будущем раздел будет анализировать товары, цены, конкурентов и прибыль;
- кнопки-заглушки: Analyze Products, Generate Recommendations.

### 8.3 Products Catalog

Цель: управление локальным каталогом товаров.

Функции первой версии:

- список товаров;
- поиск по названию, SKU, EAN;
- фильтр по статусу и категории;
- сортировка по названию, цене, остатку;
- переход в карточку товара;
- кнопка добавления товара;
- empty state, если товаров нет.

### 8.4 Product Details

Цель: посмотреть ключевую информацию о товаре.

Содержимое:

- название, SKU, статус;
- цена продажи, закупочная цена, остаток;
- ссылка на Allegro;
- заметки;
- связанные competitor offers, если есть;
- placeholder будущих блоков: прибыльность, SEO, рекомендации.

### 8.5 Add Product

Цель: быстро добавить товар вручную.

Поля формы:

- name;
- sku;
- Allegro offer ID;
- EAN;
- category;
- brand;
- purchase price;
- selling price;
- stock quantity;
- min stock quantity;
- URL;
- notes.

### 8.6 Competitors Catalog

Цель: хранить список конкурентов.

Функции первой версии:

- список конкурентов;
- поиск по названию и URL;
- фильтр по marketplace;
- кнопка добавления конкурента;
- просмотр базовой информации.

### 8.7 Add Competitor

Поля формы:

- name;
- marketplace;
- url;
- rating;
- review count;
- notes.

### 8.8 SEO

Placeholder будущего SEO-раздела:

- анализ названий;
- ключевые слова;
- качество описаний;
- рекомендации по улучшению офферов.

### 8.9 Market

Placeholder будущего анализа рынка:

- тренды;
- категории;
- рыночные цены;
- спрос.

### 8.10 Sales

Placeholder будущих продаж:

- список заказов;
- динамика продаж;
- товары-лидеры;
- товары без продаж.

### 8.11 Finance

Placeholder будущих финансов:

- доход;
- расходы;
- маржа;
- комиссия Allegro;
- прибыль.

### 8.12 Purchases

Placeholder будущих закупок:

- поставщики;
- закупочные цены;
- история закупок;
- планирование пополнения.

### 8.13 Recommendations

Placeholder будущих рекомендаций:

- что поднять в цене;
- что снизить;
- какие товары требуют внимания;
- что заказать;
- какие офферы улучшить.

### 8.14 Tasks

Минимальная версия может быть простым списком задач. Если времени мало, допускается placeholder, но архитектурно таблица `tasks` уже должна быть предусмотрена.

### 8.15 Imports

Placeholder будущих импортов:

- импорт XML/CSV;
- импорт товаров;
- импорт продаж;
- история импортов.

### 8.16 Exports

Placeholder будущих экспортов:

- экспорт товаров;
- экспорт отчетов;
- экспорт рекомендаций;
- backup локальной базы.

### 8.17 Settings

Функции первой версии:

- название магазина;
- валюта по умолчанию;
- язык интерфейса;
- тема оформления;
- путь к локальным данным/backup-информация;
- placeholder настроек Allegro API.

## 9. Переиспользуемые компоненты

### Layout-компоненты

- `AppLayout` — общий layout с sidebar и content area.
- `Sidebar` — левое меню.
- `TopBar` — заголовок текущего раздела и быстрые действия.
- `PageContainer` — единая ширина, отступы и responsive-поведение.

### UI-компоненты

- `Button`;
- `Input`;
- `Textarea`;
- `Select`;
- `Card`;
- `Badge`;
- `Tabs`;
- `Dialog`;
- `DropdownMenu`;
- `Table`;
- `Skeleton`;
- `Toast`.

### Бизнес-компоненты

- `AppPageHeader` — заголовок страницы, описание и primary action.
- `SearchInput` — единый поиск.
- `FilterBar` — контейнер фильтров.
- `DataTable` — таблицы каталога.
- `EmptyState` — красивое состояние пустого раздела.
- `ComingSoonPanel` — placeholder будущих разделов.
- `StatCard` — KPI-карточки Dashboard.
- `StatusBadge` — статусы товаров, задач, импортов.
- `EntityLink` — ссылка на связанную сущность.
- `MoneyValue` — форматирование цен.
- `DateValue` — форматирование дат.

## 10. Масштабирование проекта в будущем

### 10.1 Добавление бизнес-логики

Каждая новая функция должна добавляться внутри своего feature-модуля. Например, будущая аналитика товаров должна появиться в `features/products/analytics`, а не смешиваться с UI каталога.

### 10.2 AI Advisor

AI-слой в будущем лучше оформить как отдельный модуль:

```text
features/ai-advisor/
  api/
  prompts/
  services/
  components/
  types.ts
```

Он должен получать подготовленные данные через доменные сервисы, а не читать таблицы напрямую. Это позволит менять модель AI, провайдера или локальный/облачный режим без переписывания UI.

### 10.3 Интеграция с Allegro API

Для Allegro API нужен отдельный integration layer:

```text
integrations/allegro/
  auth/
  client/
  mappers/
  types/
```

UI не должен зависеть от формата ответов Allegro API. Все внешние данные должны проходить через mapper в внутренние модели приложения.

### 10.4 Импорты и экспорты

Импорты должны быть pipeline-based:

1. чтение файла;
2. preview;
3. mapping колонок;
4. validation;
5. apply changes;
6. import report.

Даже если первая версия показывает только placeholder, архитектура должна учитывать такой процесс.

### 10.5 Переход к PWA

Чтобы не переписывать проект:

- frontend не должен напрямую использовать Tauri API в компонентах;
- все desktop-возможности должны быть за `shared/api/client.ts` или adapter layer;
- локальная база должна быть доступна через репозитории;
- UI должен быть responsive, но оптимизирован под desktop;
- роутинг должен быть обычным web routing.

### 10.6 Возможный будущий backend

Если позже понадобится синхронизация, сервер или удаленный доступ, можно добавить backend без переписывания UI:

```text
apps/
  desktop/
  web/
  api/
packages/
  ui/
  domain/
  config/
```

Для первой версии monorepo не обязателен. Но границы модулей должны быть такими, чтобы переход был возможен.

## 11. Решения, которые помогут не переписывать проект через полгода

1. Использовать TypeScript strict mode с самого начала.
2. Не обращаться к SQLite напрямую из React-компонентов.
3. Завести repository layer для products, competitors, settings и tasks.
4. Хранить миграции базы в репозитории.
5. Использовать единые Zod-схемы для форм и валидации.
6. Вынести navigation config в один файл.
7. Делать все разделы через общий layout и page template.
8. Проектировать заглушки как настоящие страницы, а не временный текст в одном компоненте.
9. Не смешивать Allegro API types с внутренними domain types.
10. Сразу предусмотреть `created_at` и `updated_at` во всех основных таблицах.
11. Сразу предусмотреть `app_events`, чтобы в будущем понимать действия пользователя.
12. Использовать дизайн-систему и общие компоненты вместо ручной верстки каждого экрана.
13. Держать AI-функции отдельным модулем, чтобы не привязать бизнес-логику к конкретному провайдеру.
14. Делать local-first backup/export как часть будущей архитектуры.
15. Не строить multi-tenant/SaaS-архитектуру, потому что продукт персональный.

## 12. Минимальный план реализации после утверждения архитектуры

### Этап 1: Основа приложения

- создать Vite + React + TypeScript проект;
- подключить Tauri;
- настроить Tailwind и базовую дизайн-систему;
- реализовать AppLayout, Sidebar, TopBar;
- настроить маршруты всех разделов.

### Этап 2: Данные и формы

- добавить SQLite;
- добавить миграции;
- реализовать products repository;
- реализовать competitors repository;
- реализовать settings repository;
- добавить формы товара и конкурента.

### Этап 3: Основные экраны

- Dashboard;
- Products Catalog;
- Product Details;
- Add Product;
- Competitors Catalog;
- Add Competitor;
- Settings.

### Этап 4: Placeholder-разделы

- AI Advisor;
- SEO;
- Market;
- Sales;
- Finance;
- Purchases;
- Recommendations;
- Tasks;
- Imports;
- Exports.

### Этап 5: Полировка прототипа

- empty states;
- loading states;
- базовые уведомления;
- smoke-тест навигации;
- проверка desktop UX;
- подготовка списка решений для второй версии.

## 13. Критерии готовности первой версии

Первая версия считается готовой, если:

- приложение запускается на Windows как desktop-приложение;
- все пункты sidebar открывают соответствующие экраны;
- Dashboard выглядит как главный рабочий центр;
- можно добавить товар;
- можно найти и отфильтровать товары;
- можно открыть карточку товара;
- можно добавить конкурента;
- можно найти и отфильтровать конкурентов;
- Settings сохраняют базовые настройки;
- будущие разделы выглядят профессионально, даже если являются placeholder;
- структура проекта позволяет добавлять AI, Allegro API, импорты и аналитику без полной переработки.
