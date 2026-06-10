'use client';

import * as React from 'react';

type Theme = 'goat-dark' | 'minimal-dark' | 'high-contrast';

interface ThemeProviderProps {
  children: React.ReactNode;
}

interface ThemeContextType {
  theme: Theme;
  setTheme: (theme: Theme) => void;
}

const ThemeContext = React.createContext<ThemeContextType | undefined>(undefined);

export function ThemeProvider({ children }: ThemeProviderProps) {
  const [theme, setThemeState] = React.useState<Theme>('goat-dark');
  const [mounted, setMounted] = React.useState(false);

  React.useEffect(() => {
    setMounted(true);
    const savedTheme = localStorage.getItem('goat-theme') as Theme;
    if (savedTheme && ['goat-dark', 'minimal-dark', 'high-contrast'].includes(savedTheme)) {
      setThemeState(savedTheme);
    }
  }, []);

  React.useEffect(() => {
    if (!mounted) return;
    const root = document.documentElement;
    root.classList.remove('theme-goat-dark', 'theme-minimal-dark', 'theme-high-contrast');
    root.classList.add(`theme-${theme}`);
    localStorage.setItem('goat-theme', theme);
  }, [theme, mounted]);

  const setTheme = React.useCallback((newTheme: Theme) => {
    setThemeState(newTheme);
  }, []);

  // Prevent flash of incorrect theme
  if (!mounted) {
    return <div style={{ visibility: 'hidden' }}>{children}</div>;
  }

  return (
    <ThemeContext.Provider value={{ theme, setTheme }}>
      {children}
    </ThemeContext.Provider>
  );
}

export const useTheme = () => {
  const context = React.useContext(ThemeContext);
  if (!context) throw new Error('useTheme must be used within ThemeProvider');
  return context;
};
