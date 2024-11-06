export function formatDate(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  
  // Less than 24 hours
  if (diff < 24 * 60 * 60 * 1000) {
    return date.toLocaleTimeString();
  }
  
  // Less than a week
  if (diff < 7 * 24 * 60 * 60 * 1000) {
    return date.toLocaleDateString(undefined, { weekday: 'long' });
  }
  
  // Otherwise
  return date.toLocaleDateString();
} 