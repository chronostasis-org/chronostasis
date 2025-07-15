import { useMemo } from "react";
import { Item } from "../inventory-grid";

export interface UseInventoryFiltersProps {
  items: Item[];
  searchQuery: string;
  categoryFilter: string;
  rarityFilter: string;
}

export const useInventoryFilters = ({
  items,
  searchQuery,
  categoryFilter,
  rarityFilter,
}: UseInventoryFiltersProps) => {
  // Filter items based on search and filters
  const filteredItems = useMemo(() => {
    return items.filter((item) => {
      const matchesSearch = item.name
        .toLowerCase()
        .includes(searchQuery.toLowerCase());
      const matchesCategory =
        categoryFilter === "all" || item.category === categoryFilter;
      const matchesRarity =
        rarityFilter === "all" || item.rarity === rarityFilter;
      return matchesSearch && matchesCategory && matchesRarity;
    });
  }, [items, searchQuery, categoryFilter, rarityFilter]);

  // Get unique categories for filter dropdown
  const categories = useMemo(() => {
    return Array.from(new Set(items.map((item) => item.category)));
  }, [items]);

  return {
    filteredItems,
    categories,
  };
};
