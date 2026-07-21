import type { ReactNode } from 'react';
export function Card({ children, className = '' }: { children: ReactNode; className?: string }) { return <section className={`card ${className}`}>{children}</section>; }
export function PageHeader({ title, description, action }: { title: string; description: string; action?: ReactNode }) { return <div className="page-header"><div><h1>{title}</h1><p>{description}</p></div>{action}</div>; }
export function EmptyState({ title, description }: { title: string; description: string }) { return <Card className="empty"><div className="empty-icon">✦</div><h2>{title}</h2><p>{description}</p></Card>; }
export function Button(props: React.ButtonHTMLAttributes<HTMLButtonElement>) { return <button {...props} className={`btn ${props.className ?? ''}`} />; }
export function Input(props: React.InputHTMLAttributes<HTMLInputElement>) { return <input {...props} className={`input ${props.className ?? ''}`} />; }
export function Textarea(props: React.TextareaHTMLAttributes<HTMLTextAreaElement>) { return <textarea {...props} className={`input ${props.className ?? ''}`} />; }
export function Select(props: React.SelectHTMLAttributes<HTMLSelectElement>) { return <select {...props} className={`input ${props.className ?? ''}`} />; }
