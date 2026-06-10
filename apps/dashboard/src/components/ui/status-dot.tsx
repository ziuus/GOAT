import * as React from "react"
import { cn } from "./card"

export interface StatusDotProps extends React.HTMLAttributes<HTMLSpanElement> {
  status?: "online" | "offline" | "busy" | "unknown" | "pending" | "success" | "error"
  animate?: boolean
}

export function StatusDot({ status = "unknown", animate = false, className, ...props }: StatusDotProps) {
  let colorClass = "bg-zinc-500"
  
  switch (status) {
    case "online":
    case "success":
      colorClass = "bg-emerald-500"
      break
    case "offline":
    case "error":
      colorClass = "bg-red-500"
      break
    case "busy":
    case "pending":
      colorClass = "bg-amber-500"
      break
  }

  return (
    <span className={cn("relative flex h-2 w-2", className)} {...props}>
      {animate && (
        <span className={cn("absolute inline-flex h-full w-full animate-ping rounded-full opacity-75", colorClass)} />
      )}
      <span className={cn("relative inline-flex h-2 w-2 rounded-full", colorClass)} />
    </span>
  )
}
