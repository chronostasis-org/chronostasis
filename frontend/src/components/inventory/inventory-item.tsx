import React, { memo } from "react";
import { Item } from "./inventory-grid";

interface InventoryItemProps {
  item: Item;
}

const rarityColors = {
  common: "border-gray-500/60 shadow-gray-500/20",
  uncommon: "border-green-400/60 shadow-green-400/30",
  rare: "border-blue-400/60 shadow-blue-400/30",
  epic: "border-purple-400/60 shadow-purple-400/30",
  legendary: "border-yellow-400/60 shadow-yellow-400/40",
} as const;

const rarityGlow = {
  common: "shadow-sm",
  uncommon: "shadow-md shadow-green-400/20",
  rare: "shadow-md shadow-blue-400/20",
  epic: "shadow-lg shadow-purple-400/20",
  legendary: "shadow-xl shadow-yellow-400/30",
} as const;

const rarityBadgeColors = {
  common: "bg-gray-600 text-gray-100",
  uncommon: "bg-green-600 text-green-100",
  rare: "bg-blue-600 text-blue-100",
  epic: "bg-purple-600 text-purple-100",
  legendary: "bg-yellow-600 text-yellow-100",
} as const;

export const InventoryItem: React.FC<InventoryItemProps> = memo(({ item }) => {
  const isEmpty = item.quantity === 0 || item.category === "Empty";

  if (isEmpty) {
    return (
      <div className="aspect-square w-full rounded-sm border border-slate-700/50 bg-slate-800/30" />
    );
  }

  return (
    <div className="group relative aspect-square w-full">
      <div
        className={`relative h-full w-full border bg-slate-800/30 ${rarityColors[item.rarity]} ${rarityGlow[item.rarity]} cursor-pointer rounded-sm transition-all duration-200 hover:z-10 hover:brightness-110`}
      >
        {/* Item Image */}
        <div className="absolute inset-0.5 flex items-center justify-center">
          {typeof item.image === "string" ? (
            <img
              src={item.image}
              alt={item.name}
              className="h-full w-full rounded-sm object-contain"
              loading="lazy"
            />
          ) : (
            <div className="flex h-8 w-8 items-center justify-center">
              <div
                className="h-8 w-8 scale-150"
                style={{
                  backgroundImage: `url(${item.image.src})`,
                  backgroundPosition: `-${item.image.x}px -${item.image.y}px`,
                  backgroundSize: "192px 192px",
                  backgroundRepeat: "no-repeat",
                  imageRendering: "pixelated",
                }}
              />
            </div>
          )}
        </div>

        {/* Quantity Counter */}
        <div className="absolute -right-0.5 -bottom-0.5 flex h-4 w-4 items-center justify-center rounded-full border border-slate-600/50 bg-slate-900/90">
          <span className="text-xs leading-none font-bold text-white">
            {item.quantity > 99 ? "99+" : item.quantity}
          </span>
        </div>
      </div>

      {/* Tooltip on Hover */}
      {/* <div className="pointer-events-none absolute bottom-full left-1/2 z-50 mb-2 -translate-x-1/2 transform opacity-0 transition-opacity duration-300 group-hover:opacity-100">
        <div className="min-w-48 rounded-md border border-slate-600/50 bg-slate-900/95 p-3 shadow-2xl backdrop-blur-sm">
          <div className="mb-2 flex items-center gap-2">
            <h3 className="text-sm font-semibold text-white">{item.name}</h3>
            <span
              className={`rounded-full px-2 py-0.5 text-xs ${rarityBadgeColors[item.rarity]}`}
            >
              {item.rarity.toUpperCase()}
            </span>
          </div>
          <div className="space-y-1 text-xs text-slate-300">
            <div className="flex justify-between">
              <span>Category:</span>
              <span className="text-slate-100">{item.category}</span>
            </div>
            <div className="flex justify-between">
              <span>Quantity:</span>
              <span className="text-slate-100">{item.quantity}</span>
            </div>
            <div className="flex justify-between">
              <span>Last Updated:</span>
              <span className="text-slate-100">
                {new Date(item.lastUpdated).toLocaleDateString()}
              </span>
            </div>
          </div>
        </div>
      </div> */}
    </div>
  );
});

InventoryItem.displayName = "InventoryItem";
