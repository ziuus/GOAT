import * as React from "react"
import { cn } from "./card"
import { LucideIcon } from "lucide-react"

interface EmptyStateProps extends React.HTMLAttributes<HTMLDivElement> {
  icon?: LucideIcon
  title: string
  description?: string
  action?: React.ReactNode
}

export function EmptyState({
  icon: Icon,
  title,
  description,
  action,
  className,
  ...props
}: EmptyStateProps) {
  return (
    <div
      className={cn(
        "flex min-h-[400px] flex-col items-center justify-center rounded-xl border border-dashed border-zinc-800 bg-zinc-950/50 p-8 text-center animate-in fade-in-50",
        className
      )}
      {...props}
    >
      <div className="mx-auto flex max-w-[420px] flex-col items-center justify-center text-center">
        {Icon && (
          <div className="flex h-20 w-20 items-center justify-center rounded-full bg-zinc-900 mb-4">
            <Icon className="h-10 w-10 text-zinc-500" />
          </div>
        )}
        <h3 className="mt-4 text-lg font-semibold text-zinc-100">{title}</h3>
        {description && (
          <p className="mb-4 mt-2 text-sm text-zinc-400">{description}</p>
        )}
        {action}
      </div>
    </div>
  )
}
