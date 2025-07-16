"use client";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { InventoryFilters } from "./InventoryFilters";
import { InventoryGridContainer } from "./InventoryGridContainer";
import { InventoryStats } from "./InventoryStats";
import { useInventoryFilters } from "./hooks/useInventoryFilters";
import { useInventoryPagination } from "./hooks/useInventoryPagination";
import { useResponsiveGrid } from "./hooks/useResponsiveGrid";

export type SpriteImage = { src: string; x: number; y: number };
export type Item = {
  id: string;
  name: string;
  image: string | SpriteImage;
  category: string;
  quantity: number;
  rarity: "common" | "uncommon" | "rare" | "epic" | "legendary";
  lastUpdated: string;
};

type InventoryGridProps = {
  items: Item[];
};

export function InventoryGrid({ items }: InventoryGridProps) {
  const [searchQuery, setSearchQuery] = useState("");
  const [categoryFilter, setCategoryFilter] = useState<string>("all");
  const [rarityFilter, setRarityFilter] = useState<string>("all");
  const [currentPage, setCurrentPage] = useState(1);

  const screenSize = useResponsiveGrid();

  const { filteredItems, categories } = useInventoryFilters({
    items,
    searchQuery,
    categoryFilter,
    rarityFilter,
  });

  const { paginatedItems, totalPages } = useInventoryPagination({
    items: filteredItems,
    currentPage,
    screenSize,
  });

  const handleClearFilters = () => {
    setSearchQuery("");
    setCategoryFilter("all");
    setRarityFilter("all");
    setCurrentPage(1);
  };

  return (
    <div className="flex flex-col gap-6">
      <InventoryFilters
        searchQuery={searchQuery}
        setSearchQuery={setSearchQuery}
        categoryFilter={categoryFilter}
        setCategoryFilter={setCategoryFilter}
        rarityFilter={rarityFilter}
        setRarityFilter={setRarityFilter}
        categories={categories}
        onClearFilters={handleClearFilters}
      />

      <InventoryStats
        totalItems={filteredItems.length}
        currentPage={currentPage}
        totalPages={totalPages}
        onPageChange={setCurrentPage}
      />

      {paginatedItems.length > 0 ? (
        <InventoryGridContainer
          items={paginatedItems}
          screenSize={screenSize}
        />
      ) : (
        <div className="flex h-[40vh] flex-col items-center justify-center rounded-md border border-dashed border-slate-600/50 bg-slate-900/30 p-8 text-center backdrop-blur-sm">
          <div className="mx-auto flex max-w-[420px] flex-col items-center justify-center text-center">
            <h3 className="mt-4 text-lg font-semibold text-slate-100">
              No items found
            </h3>
            <p className="mt-2 mb-4 text-sm text-slate-400">
              Try adjusting your search or filters to find what you're looking
              for.
            </p>
            <Button
              onClick={handleClearFilters}
              className="border-slate-600/50 bg-slate-800 text-slate-100 hover:bg-slate-700"
            >
              Clear filters
            </Button>
          </div>
        </div>
      )}
    </div>
  );
}
