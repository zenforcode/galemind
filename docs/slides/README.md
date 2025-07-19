# Presentation Guide

## Overview

This directory contains a comprehensive reveal.js presentation showcasing the adaptive batching library implementation, including the CORK algorithm translation from Python to Rust.

## Files

- `presentation.html` - Main reveal.js presentation
- `README.md` - This guide

## Viewing the Presentation

### Option 1: Local File (Recommended)
1. Open `presentation.html` directly in your web browser
2. The presentation loads reveal.js from CDN automatically
3. Use arrow keys or click to navigate

### Option 2: Local Server
```bash
# If you prefer serving locally
cd docs/slides/
python3 -m http.server 8000
# Then open http://localhost:8000/presentation.html
```

### Option 3: VS Code Live Server Extension
1. Install "Live Server" extension in VS Code
2. Right-click on `presentation.html`
3. Select "Open with Live Server"

## Presentation Structure

### 1. Title & Introduction (Slides 1-2)
- Project overview and motivation
- Agenda and scope

### 2. Problem Statement (Slides 3-5)
- ML serving challenges
- Traditional batching limitations
- BentoML's CORK algorithm introduction

### 3. CORK Algorithm Deep Dive (Slides 6-9)
- Core concepts and mathematical foundation
- Algorithm flow and optimization process
- Why Rust translation was chosen

### 4. Rust Implementation (Slides 10-14)
- Architecture overview
- Key components and type safety
- Async implementation details
- Custom data type examples

### 5. Performance Analysis (Slides 15-18)
- Benchmark results and comparisons
- Resource efficiency analysis
- Scalability characteristics

### 6. Key Achievements (Slides 19-23)
- Complete algorithm translation
- Production-ready implementation
- Performance improvements
- Flexible architecture

### 7. Future Roadmap (Slides 24-27)
- Short, medium, and long-term plans
- Research areas and community goals

### 8. Demo & Examples (Slides 28-30)
- Live demonstration
- Configuration examples
- Metrics dashboard

### 9. Conclusion (Slides 31-34)
- Summary of achievements
- Technical innovation highlights
- Impact and applications
- Q&A

## Navigation

### Keyboard Controls
- **Arrow Keys**: Navigate between slides
- **Space**: Next slide
- **Shift + Space**: Previous slide
- **Esc**: Overview mode
- **S**: Speaker notes (if available)
- **F**: Fullscreen mode

### Mouse Controls
- **Click**: Advance to next slide
- **Scroll**: Navigate through slides

## Customization

### Themes
The presentation uses the "white" theme. To change themes, modify the CSS link:
```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/reveal.js@4.3.1/dist/theme/[THEME].css">
```

Available themes: `white`, `black`, `league`, `beige`, `sky`, `night`, `serif`, `simple`, `solarized`

### Content Updates
1. Edit the HTML directly in `presentation.html`
2. Each `<section>` represents a slide
3. Nested `<section>` elements create vertical slide stacks

### Styling
- Custom CSS is included in the `<style>` section
- Modify colors, fonts, and layouts as needed
- The presentation includes several custom classes:
  - `.highlight-box` - Blue highlighted content
  - `.success-box` - Green success indicators
  - `.warning-box` - Orange warning/attention boxes
  - `.performance-table` - Styled tables for metrics
  - `.architecture-diagram` - Monospace diagrams

## Speaker Notes

To add speaker notes:
```html
<section>
    <h2>Slide Title</h2>
    <p>Slide content...</p>
    
    <aside class="notes">
        Speaker notes go here. These are only visible when
        pressing 'S' to enter speaker mode.
    </aside>
</section>
```

## Technical Details

### Dependencies
- **Reveal.js 4.3.1**: Core presentation framework
- **Highlight.js**: Code syntax highlighting
- **CDN Delivery**: All dependencies loaded from CDN

### Browser Compatibility
- Chrome/Chromium (recommended)
- Firefox
- Safari
- Edge

### Performance
- Lightweight: ~2MB total including images and fonts
- Fast loading: CDN-delivered assets
- Responsive: Works on desktop, tablet, and mobile

## Presentation Tips

### For Speakers
1. **Practice Navigation**: Familiarize yourself with slide transitions
2. **Code Examples**: Use the presentation's code highlighting effectively
3. **Live Demo**: Consider running actual examples alongside slides
4. **Metrics Focus**: Emphasize the performance improvements achieved
5. **Interactive Elements**: Engage audience with questions about their use cases

### For Audiences
1. **Technical Depth**: Presentation assumes familiarity with Rust and async programming
2. **ML Context**: Basic understanding of ML serving patterns helpful
3. **Performance Focus**: Heavy emphasis on benchmarks and optimization
4. **Implementation Details**: Includes actual code examples and architecture

## Related Documentation

- `../design/cork-algorithm.md` - Detailed algorithm documentation
- `../design/architecture.md` - System architecture details  
- `../design/performance.md` - Comprehensive performance analysis
- `../../examples/` - Working code examples
- `../../README.md` - Project overview and quick start

## Presentation Occasions

This presentation is suitable for:
- **Technical Conferences**: Systems/performance tracks
- **Rust Meetups**: Advanced Rust programming topics
- **ML Engineering Teams**: Performance optimization discussions
- **Architecture Reviews**: System design presentations
- **Open Source Showcases**: Project demonstrations

## Updates and Maintenance

To keep the presentation current:
1. Update performance metrics as benchmarks improve
2. Add new features and capabilities as they're implemented
3. Include real-world deployment experiences
4. Update the roadmap based on actual development progress

## Feedback and Contributions

The presentation is part of the project documentation and can be improved through:
- Content suggestions and corrections
- Visual design improvements
- Additional examples and use cases
- Speaker notes and presentation tips

## Export Options

### PDF Export
1. Open presentation in Chrome
2. Add `?print-pdf` to the URL
3. Use Chrome's print function with "Save as PDF"
4. Adjust print settings for optimal layout

### Static Images
- Use browser developer tools to capture individual slides
- Consider automated screenshot tools for batch export

This presentation effectively communicates the technical achievements and practical benefits of the adaptive batching library, making it suitable for technical audiences interested in high-performance systems and Rust programming.
