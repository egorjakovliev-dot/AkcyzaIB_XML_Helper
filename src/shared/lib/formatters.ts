export const money = (value:number, currency = 'PLN') => new Intl.NumberFormat('pl-PL', { style: 'currency', currency }).format(value || 0);
