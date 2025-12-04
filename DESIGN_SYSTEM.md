# Ask Pete Design System

**Purpose**: Transform the "high school race car" aesthetic into a professional, cohesive educational platform worthy of a university research project.

---

## 🎨 Current Problem

**Student feedback**: "Looks like a high school project with a race car philosophy and engine"

**Root causes**:

1. **Railroad/train metaphor feels forced** - Mixing trains with academic learning is confusing
2. **Inconsistent visual language** - Some components modern, some feel dated
3. **No clear brand identity** - What IS Ask Pete? A game? A tool? An LMS?
4. **Overwhelming terminology** - "Train Yard", "Coal", "Boilermaker Gold" etc. alienates users

---

## 🎯 Design Philosophy (REVISED)

### What Ask Pete Actually Is

**An AI-powered Socratic learning companion that makes hard subjects accessible through story-driven experiences.**

### Core Principles

1. **Clarity over Metaphor** - Only use train imagery where it HELPS, not everywhere
2. **Academic + Modern** - Feel like a cutting-edge research tool, not a children's game
3. **Content-First** - UI supports the learning, doesn't distract from it
4. **Trust through Polish** - Every detail says "This is a serious educational tool"

---

## 🎨 Visual Identity 2.0

### Color Palette (REFINED)

**Primary Colors** (Use sparingly for impact):

```css
--primary-accent: #6366F1;      /* Indigo - AI/Tech feel */
--primary-dark: #4F46E5;        /* Darker indigo for hover */
--success: #10B981;             /* Green - success states */
--warning: #F59E0B;             /* Amber - caution */
--error: #EF4444;               /* Red - errors */
```

**Neutral Base** (Use for 90% of UI):

```css
--bg-primary: #0F172A;          /* Slate 900 - main background */
--bg-secondary: #1E293B;        /* Slate 800 - cards/panels */
--bg-tertiary: #334155;         /* Slate 700 - hover states */
--text-primary: #F1F5F9;        /* Slate 100 - main text */
--text-secondary: #CBD5E1;      /* Slate 300 - secondary text */
--text-muted: #64748B;          /* Slate 500 - labels */
--border: #475569;              /* Slate 600 - borders */
```

**REMOVE** (Too "gimmicky"):

- ❌ "Boilermaker Gold" (#CFB991) - Feels like a sports team
- ❌ "Railyard Dark" - Too thematic, not professional
- ❌ Train track background patterns - Distracting

**REPLACE WITH**:

- ✅ Subtle grid patterns (like VS Code)
- ✅ Clean gradients (Tailwind-style)
- ✅ Professional UI conventions

---

## 🏗️ Component Design Standards

### Buttons

**Current**: Multiple inconsistent styles  
**Solution**: 3 standard button types

```css
/* Primary Action - Use for main CTA*/
.btn-primary {
  background: linear-gradient(to-br, #6366F1, #4F46E5);
  color: white;
  padding: 0.625rem 1.25rem;
  border-radius: 0.5rem;
  font-weight: 600;
  transition: all 0.2s;
  box-shadow: 0 1px 3px rgba(99, 102, 241, 0.2);
}
.btn-primary:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 8px rgba(99, 102, 241, 0.3);
}

/* Secondary Action */
.btn-secondary {
  background: #1E293B;
  color: #F1F5F9;
  border: 1px solid #475569;
  padding: 0.625rem 1.25rem;
  border-radius: 0.5rem;
  font-weight: 500;
  transition: all 0.2s;
}
.btn-secondary:hover {
  background: #334155;
  border-color: #64748B;
}

/* Destructive/Warning */
.btn-danger {
  background: #DC2626;
  color: white;
  padding: 0.625rem 1.25rem;
  border-radius: 0.5rem;
  font-weight: 600;
}
```

### Cards

**Current**: Inconsistent spacing, borders, shadows  
**Solution**: Standard card component

```css
.card {
  background: #1E293B;
  border: 1px solid #334155;
  border-radius: 0.75rem;
  padding: 1.5rem;
  transition: all 0.2s;
}
.card:hover {
  border-color: #6366F1;
  box-shadow: 0 4px 12px rgba(99, 102, 241, 0.1);
}

.card-header {
  font-size: 1.25rem;
  font-weight: 600;
  color: #F1F5F9;
  margin-bottom: 0.5rem;
}

.card-body {
  color: #CBD5E1;
  line-height: 1.6;
}
```

### Input Fields

```css
.input {
  background: #0F172A;
  border: 1px solid #475569;
  color: #F1F5F9;
  padding: 0.625rem 0.875rem;
  border-radius: 0.5rem;
  transition: all 0.2s;
}
.input:focus {
  border-color: #6366F1;
  outline: none;
  box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
}
```

---

## 📐 Layout Standards

### Spacing Scale (Consistent everywhere)

```
--space-xs: 0.25rem;    /* 4px */
--space-sm: 0.5rem;     /* 8px */
--space-md: 1rem;       /* 16px */
--space-lg: 1.5rem;     /* 24px */
--space-xl: 2rem;       /* 32px */
--space-2xl: 3rem;      /* 48px */
```

### Typography Scale

```css
.text-xs { font-size: 0.75rem; line-height: 1rem; }    /* 12px */
.text-sm { font-size: 0.875rem; line-height: 1.25rem; } /* 14px */
.text-base { font-size: 1rem; line-height: 1.5rem; }    /* 16px */
.text-lg { font-size: 1.125rem; line-height: 1.75rem; } /* 18px */
.text-xl { font-size: 1.25rem; line-height: 1.75rem; }  /* 20px */
.text-2xl { font-size: 1.5rem; line-height: 2rem; }     /* 24px */
.text-3xl { font-size: 1.875rem; line-height: 2.25rem; }/* 30px */
```

---

## 🎯 Page-Specific Redesigns

### Teacher Dashboard/NodeCanvas

**Current Issues**:

- Train track connections feel childish
- "🚉 Train Yard is Empty" messaging is confusing
- Too much railroad metaphor

**Redesign**:

- **Connections**: Clean Bezier curves (like Figma/n8n), not railroad tracks
- **Empty State**: "Start Building Your Story" not "Train Yard is Empty"
- **Toolbar**: Modern icon buttons (Material Design style)
- **Background**: Subtle dot grid, not railroad ties

### Student Dashboard/Scenario Selection

**Current Issues**:

- Scenarios feel like "levels in a game"
- Not clear this is educational content

**Redesign**:

- **Cards**: Clean, academic-looking cards with:
  - Clear subject tags (Math, Science, etc.)
  - Difficulty level (subtle, not game-like)
  - Learning outcomes visible
- **Typography**: Bigger, cleaner fonts
- **Imagery**: Use icons, not train emojis

---

## 🚀 Quick Wins (Implement First)

### 1. Create a Shared Tailwind Config

**File**: `tailwind.config.js` (in project root)

```javascript
module.exports = {
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: '#6366F1',
          dark: '#4F46E5',
        },
        slate: {
          950: '#0F172A',
        }
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
      },
    },
  },
}
```

### 2. Import Professional Font

**File**: `index.html` (in each app)

```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet">
```

### 3. Standard Button Component

**File**: `apps/teacher/src/components/ui/button.rs`

```rust
#[component]
pub fn Button(
    #[prop(into)] variant: String, // "primary" | "secondary" | "danger"
    #[prop(into)] on_click: Callback<()>,
    children: Children,
) -> impl IntoView {
    let base_classes = "px-5 py-2.5 rounded-lg font-semibold transition-all";
    let variant_classes = match variant.as_str() {
        "primary" => "bg-gradient-to-br from-indigo-600 to-indigo-700 text-white hover:shadow-lg hover:-translate-y-0.5",
        "secondary" => "bg-slate-800 text-slate-100 border border-slate-600 hover:bg-slate-700",
        "danger" => "bg-red-600 text-white hover:bg-red-700",
        _ => "bg-slate-700 text-slate-200"
    };
    
    view! {
        <button 
            class=format!("{} {}", base_classes, variant_classes)
            on:click=move |_| on_click.run(())
        >
            {children()}
        </button>
    }
}
```

---

## 📊 Before/After Examples

### NodeCanvas Toolbar

**Before**:

```
[Ask Pete Authoring] [▶ Play] [💾 Save] [🦉 O.W.L.] [📐 Blueprint]
```

**After**:

```
Ask Pete Studio

[▶ Test]  [💾 Save]  [🤖 AI Assist]  [⚙️ Settings]

Clean, consistent icons. Professional spacing. Clear hierarchy.
```

### Empty State

**Before**:

```
🚉
The Train Yard is Empty
Click 'Design with Pete' to start building your route
```

**After**:

```
📝
Start Building Your Story
Create your first learning scenario using AI-assisted authoring

[+ New Scenario]  [Browse Templates]
```

---

## 🎓 Design References (Study These)

1. **Notion** - Clean, professional, academic feel
2. **Linear** - Modern SaaS UI, great button/card design
3. **Vercel** - Excellent dark mode implementation
4. **VS Code** - Professional developer tool aesthetic
5. **Figma** - Node-based editor (for NodeCanvas inspiration)

**NOT**:

- ❌ Game UIs (too playful)
- ❌ Children's educational software (too simplistic)
- ❌ Heavy metaphor UIs (confusing)

---

## ✅ Implementation Checklist

### Phase 1: Foundation (1-2 hours)

- [ ] Add Inter font to all apps
- [ ] Create shared Tailwind config
- [ ] Document color palette in code
- [ ] Create `Button` component with 3 variants
- [ ] Create `Card` component

### Phase 2: NodeCanvas Redesign (2-3 hours)

- [ ] Replace train track connections with clean Bezier curves
- [ ] Update toolbar with consistent button style
- [ ] New empty state messaging
- [ ] Remove railroad background pattern
- [ ] Add subtle dot grid background

### Phase 3: Student Dashboard (1-2 hours)

- [ ] Redesign scenario cards (cleaner, more academic)
- [ ] Better typography hierarchy
- [ ] Add subject/difficulty tags
- [ ] Professional hover states

### Phase 4: Polish (1 hour)

- [ ] Consistent spacing throughout
- [ ] Smooth transitions on all interactive elements
- [ ] Loading states
- [ ] Error states

---

## 💡 Key Principle

**"Every pixel should communicate: This is a serious, research-backed educational tool, not a hobby project."**

Remove whimsy. Add professionalism. Keep the AI power.
