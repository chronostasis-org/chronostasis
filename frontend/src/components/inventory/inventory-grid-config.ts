// Grid configuration for different screen sizes
export const GRID_CONFIG = {
  mobile: {
    cols: 5,
    itemsPerPage: 30,
    gap: 0,
  },
  tablet: {
    cols: 8,
    itemsPerPage: 32,
    gap: 0,
  },
  desktop: {
    cols: 12,
    itemsPerPage: 64,
    gap: 0,
  },
} as const;

// Responsive grid classes
export const RESPONSIVE_GRID_CLASSES = `grid-cols-${GRID_CONFIG.mobile.cols} sm:grid-cols-${GRID_CONFIG.tablet.cols} lg:grid-cols-${GRID_CONFIG.desktop.cols}`;

// Grid container classes
export const GRID_CONTAINER_CLASSES =
  "grid gap-2 bg-black p-2 rounded-lg border border-slate-600/50 backdrop-blur-sm";
