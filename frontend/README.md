# Lontar Balinese Dictionary - Frontend

The public web interface for the Lontar Balinese Dictionary (LBD), built with SvelteKit.

## Overview

This frontend provides a user-friendly interface for searching and browsing the comprehensive Balinese-Indonesian-English dictionary with etymological information and corpus attestations.

## Features

### Phase 1.5 Features
- **Search**: Instant search with debouncing, multi-language support (Bali/ID/EN)
- **Entry Pages**: Detailed dictionary entries with senses, etymology, attestations
- **Browse**: Alphabetical browsing of dictionary entries
- **i18n**: UI language toggle (English/Indonesian)
- **Responsive**: Mobile-friendly design
- **Accessibility**: Semantic HTML and keyboard navigation

### Technical Features
- **Framework**: SvelteKit with TypeScript
- **Styling**: Custom CSS with CSS variables for theming
- **Search**: Client-side search via Meilisearch API
- **Fonts**: Noto Serif Balinese preloaded for Phase 2 Aksara support
- **Performance**: Optimized for fast loading and smooth interactions

## Development

### Prerequisites
- Node.js 18+
- pnpm package manager

### Setup

```bash
# Install dependencies
pnpm install

# Start development server
pnpm dev

# Build for production
pnpm build

# Preview production build
pnpm preview
```

### Environment Variables

Create a `.env.local` file:

```env
PUBLIC_API_BASE_URL=http://localhost:3000/api
```

### Project Structure

```
src/
├── lib/
│   ├── api.ts          # API client for backend communication
│   └── stores.ts       # Svelte stores for state management
├── routes/
│   ├── +page.svelte    # Home/search page
│   ├── entry/[lemma]/  # Individual entry pages
│   ├── browse/         # Alphabetical browsing
│   └── about/          # About page
├── app.html            # HTML template with font preloading
├── app.css             # Global styles and CSS variables
└── app.ts              # Type definitions and translations
```

## API Integration

The frontend communicates with the Rust backend via a REST API:

- `GET /entries/search?q=&lang=&filter=` - Search entries
- `GET /entries/:lemma` - Get specific entry
- `GET /entries/:id/attestations` - Get entry attestations
- `GET /corpus/:id` - Get corpus information

## Internationalization

The UI supports English and Indonesian languages:

```typescript
// Language switching
import { currentLanguage } from '$lib/stores';
currentLanguage.set('id'); // Switch to Indonesian
```

Translations are defined in `src/app.ts` with type safety for all UI strings.

## Styling System

Uses CSS custom properties for consistent theming:

```css
:root {
  --color-primary: #2563eb;
  --color-secondary: #64748b;
  --background: #ffffff;
  --surface: #f8fafc;
  --text-primary: #1e293b;
  --text-secondary: #64748b;
  --border: #e2e8f0;
  --font-bali: 'Noto Serif Balinese', serif;
}
```

## Accessibility

- Semantic HTML5 structure
- ARIA labels where appropriate
- Keyboard navigation support
- Screen reader compatibility
- High contrast color scheme

## Performance

- Minimal JavaScript footprint with SvelteKit
- Font preloading for Balinese script
- Debounced search to reduce API calls
- Optimized CSS with minimal redundancy
- Responsive images and layouts

## Future Phases

### Phase 2 - Aksara Bali Rendering
- Unicode Balinese script display
- Aksara search input support
- Font subsetting for performance

### Phase 3+ - Enhanced Features
- Corpus manuscript viewer
- Advanced search filters
- User authentication for contributions
- Editorial interface integration

## Contributing

1. Follow the existing code style and patterns
2. Ensure TypeScript types are properly defined
3. Test accessibility with screen readers
4. Verify responsive design on mobile devices
5. Check performance impact of changes

## License

This frontend is part of the Lontar Balinese Dictionary project. See the main project license for details.
