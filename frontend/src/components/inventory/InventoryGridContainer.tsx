import React from "react";
import { InventoryItem } from "./inventory-item";
import { Item } from "./inventory-grid";
import {
  GRID_CONFIG,
  RESPONSIVE_GRID_CLASSES,
  GRID_CONTAINER_CLASSES,
} from "./inventory-grid-config";
import { ScreenSize } from "./hooks/useResponsiveGrid";

export interface InventoryGridContainerProps {
  items: Item[];
  screenSize: ScreenSize;
}

export const InventoryGridContainer: React.FC<InventoryGridContainerProps> = ({
  items,
  screenSize,
}) => {
  const gridCols = GRID_CONFIG[screenSize].cols;

  return (
    <div
      className={`${GRID_CONTAINER_CLASSES} grid-cols-${gridCols}`}
      style={{
        display: "grid",
        gridTemplateColumns: `repeat(${gridCols}, 1fr)`,
      }}
    >
      {items.map((item) => (
        <InventoryItem key={item.id} item={item} />
      ))}
    </div>
  );
};
