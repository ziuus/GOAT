import * as React from "react"
import { cn } from "./card"

export interface BadgeProps extends React.HTMLAttributes<HTMLDivElement> {
  variant?: "default" | "secondary" | "destructive" | "outline"
}

function Badge({ className, variant = "default", ...props }: BadgeProps) {
  let variantClasses = "border-transparent bg-zinc-100 text-zinc-900 hover:bg-zinc-100/80"
  if (variant === "secondary") variantClasses = "border-transparent bg-zinc-800 text-zinc-100 hover:bg-zinc-800/80"
  if (variant === "destructive") variantClasses = "border-transparent bg-red-900 text-zinc-100 hover:bg-red-900/80"
  if (variant === "outline") variantClasses = "text-zinc-100"

  return (
    <div
      className={cn(
        "inline-flex items-center rounded-md border border-zinc-800 px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-zinc-700 focus:ring-offset-2",
        variantClasses,
        className
      )}
      {...props}
    />
  )
}

export { Badge }
