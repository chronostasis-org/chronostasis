import React from "react";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Pagination,
  PaginationContent,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from "@/components/ui/pagination";

export interface InventoryStatsProps {
  totalItems: number;
  currentPage: number;
  totalPages: number;
  onPageChange: (page: number) => void;
}

export const InventoryStats: React.FC<InventoryStatsProps> = ({
  totalItems,
  currentPage,
  totalPages,
  onPageChange,
}) => {
  return (
    <div className="flex flex-col gap-4">
      {/* Stats */}
      <div className="flex items-center justify-between">
        <Badge
          variant="outline"
          className="border-slate-600/50 bg-slate-800/30 text-xs text-slate-100"
        >
          {totalItems} items found
        </Badge>

        {totalPages > 1 && (
          <div className="text-xs text-slate-400">
            Page {currentPage} of {totalPages}
          </div>
        )}
      </div>

      {/* Pagination */}
      {totalPages > 1 && (
        <Pagination>
          <PaginationContent>
            <PaginationItem>
              <PaginationPrevious
                href="#"
                onClick={(e) => {
                  e.preventDefault();
                  onPageChange(Math.max(currentPage - 1, 1));
                }}
                aria-disabled={currentPage === 1}
                className={
                  currentPage === 1
                    ? "pointer-events-none text-slate-500 opacity-50"
                    : "text-slate-300 hover:text-slate-100"
                }
              />
            </PaginationItem>

            {Array.from({ length: totalPages }).map((_, i) => (
              <PaginationItem key={i}>
                <PaginationLink
                  href="#"
                  onClick={(e) => {
                    e.preventDefault();
                    onPageChange(i + 1);
                  }}
                  isActive={currentPage === i + 1}
                  className={
                    currentPage === i + 1
                      ? "border-slate-500 bg-slate-700 text-slate-100"
                      : "text-slate-300 hover:bg-slate-800 hover:text-slate-100"
                  }
                >
                  {i + 1}
                </PaginationLink>
              </PaginationItem>
            ))}

            <PaginationItem>
              <PaginationNext
                href="#"
                onClick={(e) => {
                  e.preventDefault();
                  onPageChange(Math.min(currentPage + 1, totalPages));
                }}
                aria-disabled={currentPage === totalPages}
                className={
                  currentPage === totalPages
                    ? "pointer-events-none text-slate-500 opacity-50"
                    : "text-slate-300 hover:text-slate-100"
                }
              />
            </PaginationItem>
          </PaginationContent>
        </Pagination>
      )}
    </div>
  );
};
