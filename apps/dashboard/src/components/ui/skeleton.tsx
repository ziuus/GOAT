import * as React from "react"
import { cn } from "./card"

function Skeleton({
  className,
  ...props
}: React.HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn("animate-pulse rounded-md bg-zinc-800", className)}
      {...props}
    />
  )
}

export { Skeleton }
