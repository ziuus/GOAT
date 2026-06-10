import * as React from "react"
import { cn } from "./card"

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "default" | "destructive" | "outline" | "secondary" | "ghost" | "link"
  size?: "default" | "sm" | "lg" | "icon"
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant = "default", size = "default", ...props }, ref) => {
    
    let variantClasses = "bg-zinc-100 text-zinc-900 hover:bg-zinc-200"
    if (variant === "destructive") variantClasses = "bg-red-900 text-red-100 hover:bg-red-900/90"
    if (variant === "outline") variantClasses = "border border-zinc-800 bg-transparent hover:bg-zinc-800 text-zinc-100"
    if (variant === "secondary") variantClasses = "bg-zinc-800 text-zinc-100 hover:bg-zinc-800/80"
    if (variant === "ghost") variantClasses = "hover:bg-zinc-800 text-zinc-100"
    if (variant === "link") variantClasses = "text-zinc-100 underline-offset-4 hover:underline"

    let sizeClasses = "h-9 px-4 py-2"
    if (size === "sm") sizeClasses = "h-8 rounded-md px-3 text-xs"
    if (size === "lg") sizeClasses = "h-10 rounded-md px-8"
    if (size === "icon") sizeClasses = "h-9 w-9"

    return (
      <button
        className={cn(
          "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-zinc-700 disabled:pointer-events-none disabled:opacity-50",
          variantClasses,
          sizeClasses,
          className
        )}
        ref={ref}
        {...props}
      />
    )
  }
)
Button.displayName = "Button"

export { Button }
