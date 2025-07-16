import { useMemo } from "react";
import { Item } from "../inventory-grid";
import { GRID_CONFIG } from "../inventory-grid-config";

export interface UseInventoryPaginationProps {
  items: Item[];
  currentPage: number;
  screenSize: "mobile" | "tablet" | "desktop";
}

export const useInventoryPagination = ({
  items,
  currentPage,
  screenSize,
}: UseInventoryPaginationProps) => {
  const itemsPerPage = GRID_CONFIG[screenSize].itemsPerPage;
  const totalSlots =
    GRID_CONFIG[screenSize].cols *
    Math.ceil(itemsPerPage / GRID_CONFIG[screenSize].cols);

  // Calculate pagination
  const totalPages = Math.ceil(items.length / itemsPerPage);

  // Get current page items and fill empty slots
  const paginatedItems = useMemo(() => {
    const startIndex = (currentPage - 1) * itemsPerPage;
    const pageItems = items.slice(startIndex, startIndex + itemsPerPage);

    // Fill remaining slots with empty items for visual consistency
    const emptySlots = totalSlots - pageItems.length;
    const emptyItems: Item[] = Array(emptySlots)
      .fill(null)
      .map((_, index) => ({
        id: `empty-${startIndex + pageItems.length + index}`,
        name: "",
        image: { src: "", x: 0, y: 0 },
        category: "Empty",
        quantity: 0,
        rarity: "common" as const,
        lastUpdated: new Date().toISOString(),
      }));

    return [...pageItems, ...emptyItems];
  }, [items, currentPage, itemsPerPage, totalSlots]);

  return {
    paginatedItems,
    totalPages,
    itemsPerPage,
  };
};
