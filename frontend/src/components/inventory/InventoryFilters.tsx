import React from "react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Search, SlidersHorizontal } from "lucide-react";

export interface InventoryFiltersProps {
  searchQuery: string;
  setSearchQuery: (query: string) => void;
  categoryFilter: string;
  setCategoryFilter: (category: string) => void;
  rarityFilter: string;
  setRarityFilter: (rarity: string) => void;
  categories: string[];
  onClearFilters: () => void;
}

export const InventoryFilters: React.FC<InventoryFiltersProps> = ({
  searchQuery,
  setSearchQuery,
  categoryFilter,
  setCategoryFilter,
  rarityFilter,
  setRarityFilter,
  categories,
  onClearFilters,
}) => {
  return (
    <div className="flex flex-col gap-4 rounded-lg border border-slate-600/50 bg-slate-800/30 p-4 backdrop-blur-sm sm:flex-row sm:items-center sm:justify-between">
      <div className="relative flex-1">
        <Search className="absolute top-2.5 left-2.5 h-4 w-4 text-slate-400" />
        <Input
          placeholder="Search inventory..."
          className="border-slate-600/50 bg-slate-900/50 pl-8 text-slate-100 placeholder:text-slate-400 focus:border-blue-500"
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
        />
      </div>

      <div className="flex items-center gap-2">
        <Select value={categoryFilter} onValueChange={setCategoryFilter}>
          <SelectTrigger className="w-[150px] border-slate-600/50 bg-slate-900/50 text-slate-100">
            <SelectValue placeholder="Category" />
          </SelectTrigger>
          <SelectContent className="border-slate-600/50 bg-slate-800">
            <SelectItem
              value="all"
              className="text-slate-100 focus:bg-slate-700"
            >
              All Categories
            </SelectItem>
            {categories.map((category) => (
              <SelectItem
                key={category}
                value={category}
                className="text-slate-100 focus:bg-slate-700"
              >
                {category}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>

        <Select value={rarityFilter} onValueChange={setRarityFilter}>
          <SelectTrigger className="w-[150px] border-slate-600/50 bg-slate-900/50 text-slate-100">
            <SelectValue placeholder="Rarity" />
          </SelectTrigger>
          <SelectContent className="border-slate-600/50 bg-slate-800">
            <SelectItem
              value="all"
              className="text-slate-100 focus:bg-slate-700"
            >
              All Rarity
            </SelectItem>
            <SelectItem
              value="common"
              className="text-slate-100 focus:bg-slate-700"
            >
              Common
            </SelectItem>
            <SelectItem
              value="uncommon"
              className="text-slate-100 focus:bg-slate-700"
            >
              Uncommon
            </SelectItem>
            <SelectItem
              value="rare"
              className="text-slate-100 focus:bg-slate-700"
            >
              Rare
            </SelectItem>
            <SelectItem
              value="epic"
              className="text-slate-100 focus:bg-slate-700"
            >
              Epic
            </SelectItem>
            <SelectItem
              value="legendary"
              className="text-slate-100 focus:bg-slate-700"
            >
              Legendary
            </SelectItem>
          </SelectContent>
        </Select>

        <Button
          variant="outline"
          size="icon"
          className="border-slate-600/50 bg-slate-900/50 text-slate-100 hover:bg-slate-700"
          onClick={onClearFilters}
        >
          <SlidersHorizontal className="h-4 w-4" />
        </Button>
      </div>
    </div>
  );
};
