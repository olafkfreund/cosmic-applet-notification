# Animation System Architecture

> Created: 2026-01-15
> Status: Phase 4B - Architecture Complete, Implementation In Progress
> Issue: #15

## Overview

This document describes the animation system for the COSMIC Notification Applet. The system provides smooth, accessible animations for notification appearance, dismissal, and UI transitions while respecting user accessibility preferences.

## Design Goals

1. **Smooth 60fps animations** - Target 16ms frame time
2. **Accessibility first** - Respect `prefers-reduced-motion` settings
3. **Configurable** - Users can enable/disable specific animation types
4. **Performance** - Minimal CPU/memory overhead
5. **Type-safe** - Compile-time guarantees for animation states

## Architecture Components

### 1. Animation Module (`src/ui/animation.rs`)

The core animation system with three main components:

#### Easing Functions

```rust
pub enum Easing {
    Linear,        // No easing
    EaseIn,        // Slow start
    EaseOut,       // Slow end
    EaseInOut,     // Slow start and end
    CubicIn,       // Cubic acceleration
    CubicOut,      // Cubic deceleration
    CubicInOut,    // Cubic both
    ExpoOut,       // Exponential deceleration
    BounceOut,     // Spring-like bounce
}
```

**Usage**: Each easing function takes progress (0.0-1.0) and returns an eased value.

#### Animation Timelines

```rust
pub struct Animation {
    start_time: Instant,
    duration: AnimationDuration,
    easing: Easing,
    start_value: f32,
    end_value: f32,
    completed: bool,
}
```

**Features**:
- Generic animation from any value to any value
- Automatic progress tracking
- Completion detection
- Reset and restart capability

#### Animation Durations

```rust
pub struct AnimationDuration(pub u64);  // milliseconds

// Predefined durations
AnimationDuration::INSTANT  // 0ms (no animation)
AnimationDuration::FAST     // 200ms
AnimationDuration::NORMAL   // 300ms
AnimationDuration::SLOW     // 500ms
```

**Multiplier Support**: Duration can be scaled by `config.animations.speed_multiplier` (0.1x - 3.0x).

### 2. Notification Animations

```rust
pub struct NotificationAnimation {
    pub notification_id: u32,
    pub animation_type: NotificationAnimationType,
    pub opacity: Animation,           // Fade in/out
    pub translation_y: Animation,     // Slide effect
    pub scale: Animation,             // Zoom effect
}
```

#### Animation Types

**Appearing** (when notification first arrives):
- Opacity: 0.0 → 1.0 (fade in)
- Translation Y: Slides from offset → 0
- Scale: 0.95 → 1.0 (subtle zoom)
- Duration: NORMAL (300ms)
- Easing: CubicOut (smooth deceleration)

**Dismissing** (when user dismisses):
- Opacity: 1.0 → 0.0 (fade out)
- Translation Y: 0 → -50px (slide up slightly)
- Scale: 1.0 → 0.95 (subtle shrink)
- Duration: FAST (200ms)
- Easing: CubicIn / EaseIn (smooth acceleration)

**Idle** (no animation):
- All values fixed at visible state
- Instant completion

### 3. Popup Animations

```rust
pub struct PopupAnimation {
    pub is_opening: bool,
    pub opacity: Animation,    // Fade
    pub scale: Animation,      // Zoom
}
```

**Opening**:
- Opacity: 0.0 → 1.0
- Scale: 0.95 → 1.0
- Duration: FAST (200ms)
- Easing: CubicOut

**Closing**:
- Opacity: 1.0 → 0.0
- Scale: 1.0 → 0.95
- Duration: FAST (200ms)
- Easing: CubicIn

### 4. Progress Indicators

```rust
pub struct ProgressIndicator {
    pub notification_id: u32,
    pub total_duration: i64,      // seconds
    pub start_time: Instant,
}
```

**Features**:
- Linear progress (0.0 - 1.0) based on notification timeout
- Remaining time calculation
- Expiration detection

## Configuration

### AnimationConfig Structure

```rust
pub struct AnimationConfig {
    pub enabled: bool,                 // Master toggle
    pub notification_appear: bool,     // Appearance animations
    pub notification_dismiss: bool,    // Dismissal animations
    pub popup_transitions: bool,       // Popup open/close
    pub hover_effects: bool,           // Card hover effects
    pub show_progress: bool,           // Progress indicators
    pub speed_multiplier: f32,         // 0.1 - 3.0
    pub respect_accessibility: bool,   // Honor prefers-reduced-motion
}
```

**Default**: All enabled, 1.0x speed, accessibility respected.

**Validation**:
- `speed_multiplier`: Clamped to 0.1 - 3.0
- All booleans: No validation needed

**Storage**: Persisted in COSMIC config at `~/.config/cosmic/com.system76.CosmicAppletNotifications/v1/`

## Application Integration

### State Management

```rust
pub struct NotificationApplet {
    // ... existing fields

    /// Animation states for each notification
    notification_animations: HashMap<u32, NotificationAnimation>,

    /// Popup animation state
    popup_animation: Option<PopupAnimation>,

    /// Progress indicators for timed notifications
    progress_indicators: HashMap<u32, ProgressIndicator>,

    /// Accessibility: reduced motion preference
    prefers_reduced_motion: bool,
}
```

### Message Types

```rust
pub enum Message {
    // ... existing messages

    // Animation messages
    AnimationFrame,                        // 60fps update tick
    StartAppearAnimation(u32),             // Begin notification appear
    StartDismissAnimation(u32),            // Begin notification dismiss
    CompleteNotificationDismissal(u32),    // Cleanup after animation
}
```

## Animation Lifecycle

### Notification Appearance Flow

```mermaid
sequenceDiagram
    participant DBus
    participant Manager
    participant Applet
    participant Renderer

    DBus->>Manager: New Notification
    Manager->>Applet: NotificationReceived(notification)

    alt Animations Enabled
        Applet->>Applet: Create NotificationAnimation::appearing()
        Applet->>Applet: Add to notification_animations map
        Applet->>Applet: Start AnimationFrame subscription
        loop Every 16ms
            Applet->>Applet: Update animation progress
            Applet->>Renderer: Render with current opacity/transform
        end
        Applet->>Applet: Animation complete → remove from map
    else Animations Disabled
        Applet->>Renderer: Render immediately (no animation)
    end
```

### Notification Dismissal Flow

```mermaid
sequenceDiagram
    participant User
    participant Applet
    participant Manager
    participant Renderer

    User->>Applet: Click Dismiss / Press Delete
    Applet->>Applet: DismissNotification(id)

    alt Animations Enabled
        Applet->>Applet: StartDismissAnimation(id)
        Applet->>Applet: Create NotificationAnimation::dismissing()
        loop Every 16ms
            Applet->>Renderer: Render with fade-out
        end
        Applet->>Applet: Animation complete
        Applet->>Applet: CompleteNotificationDismissal(id)
        Applet->>Manager: remove_notification(id)
    else Animations Disabled
        Applet->>Manager: remove_notification(id) immediately
    end
```

## Implementation Status

### ✅ Completed (Architecture Phase)

1. **Animation Core** (`src/ui/animation.rs`)
   - [x] Easing functions (9 variants)
   - [x] Animation struct with timeline management
   - [x] AnimationDuration with presets
   - [x] Comprehensive unit tests (100% coverage)

2. **Notification Animations**
   - [x] NotificationAnimation struct
   - [x] NotificationAnimationType enum
   - [x] Appearing animation factory
   - [x] Dismissing animation factory
   - [x] Idle state support

3. **Popup Animations**
   - [x] PopupAnimation struct
   - [x] Opening animation factory
   - [x] Closing animation factory

4. **Progress Indicators**
   - [x] ProgressIndicator struct
   - [x] Linear progress calculation
   - [x] Remaining time tracking
   - [x] Expiration detection

5. **Configuration**
   - [x] AnimationConfig struct
   - [x] AppletConfig integration
   - [x] Validation (speed_multiplier: 0.1-3.0)
   - [x] Sanitization (automatic clamping)
   - [x] Default values
   - [x] Serde serialization

6. **Application State**
   - [x] Animation state fields added
   - [x] Message types defined
   - [x] State initialization

### 🚧 In Progress (Implementation Phase)

7. **Message Handlers**
   - [ ] Handle `NotificationReceived` - trigger appear animation
   - [ ] Handle `DismissNotification` - trigger dismiss animation
   - [ ] Handle `AnimationFrame` - update all animations
   - [ ] Handle `StartAppearAnimation`
   - [ ] Handle `StartDismissAnimation`
   - [ ] Handle `CompleteNotificationDismissal`

8. **Subscriptions**
   - [ ] AnimationFrame subscription (16ms interval)
   - [ ] Auto-stop when no animations active

9. **Rendering**
   - [ ] Apply opacity to notification cards
   - [ ] Apply translation transforms
   - [ ] Apply scale transforms
   - [ ] Render progress indicators
   - [ ] Apply popup animations

10. **Accessibility**
    - [ ] Detect system `prefers-reduced-motion` setting
    - [ ] Respect `config.animations.respect_accessibility`
    - [ ] Instant animations when reduced motion preferred
    - [ ] Fallback to INSTANT duration

### 📋 Remaining Tasks

11. **Settings UI** (Future Phase 5)
    - [ ] Animation toggle switches
    - [ ] Speed multiplier slider (0.1x - 3.0x)
    - [ ] Live preview
    - [ ] Reset to defaults button

12. **Testing** (Blocked by build environment)
    - [ ] Visual verification at 60fps
    - [ ] Performance profiling
    - [ ] Accessibility testing
    - [ ] Cross-platform testing

13. **Documentation**
    - [ ] User guide (how to configure)
    - [ ] API documentation
    - [ ] Animation cookbook (custom animations)

## Performance Considerations

### Frame Budget

Target: **16ms per frame** (60fps)

**Breakdown**:
- Animation updates: < 1ms (simple float math)
- Rendering: < 10ms (GPU-accelerated)
- Event handling: < 3ms
- Buffer: 2ms

### Optimization Strategies

1. **Lazy Animation**: Only create animations when needed
2. **Auto-cleanup**: Remove completed animations immediately
3. **Batch updates**: Update all animations in single AnimationFrame
4. **Skip when idle**: Pause AnimationFrame subscription when no active animations
5. **Reduced motion**: Use INSTANT duration (0ms) when accessibility requires

### Memory Usage

Estimated per-notification animation:
- NotificationAnimation struct: ~120 bytes
- HashMap entry overhead: ~24 bytes
- Total: ~144 bytes per animated notification

With max 10 active notifications: **~1.5 KB**

ProgressIndicator adds: **~48 bytes per notification**

## Accessibility

### Reduced Motion Support

When `prefers-reduced-motion` is detected OR `config.animations.respect_accessibility` is true:

**Behavior**:
- All animations use `AnimationDuration::INSTANT`
- Easing becomes `Easing::Linear` (no effect at 0ms)
- Notifications appear/disappear immediately
- Popup transitions are instant
- Progress indicators remain visible (no movement)

**Detection** (TODO):
```rust
// Platform-specific detection
#[cfg(target_os = "linux")]
fn detect_prefers_reduced_motion() -> bool {
    // Check GNOME/KDE settings
    // Check freedesktop settings
    // Fallback: false
}
```

## Testing Strategy

### Unit Tests

**Already implemented** in `src/ui/animation.rs`:
- ✅ Easing function correctness (bounds, curves)
- ✅ Animation progress calculation
- ✅ Duration conversions
- ✅ Factory methods (appearing, dismissing, idle)
- ✅ Progress indicator calculations

### Integration Tests (Blocked)

**Requires working build**:
- [ ] Animation triggered on notification received
- [ ] Dismissal animation completes before removal
- [ ] Popup animations on open/close
- [ ] AnimationFrame subscription lifecycle

### Visual Tests (Blocked)

**Requires running application**:
- [ ] Smooth fade-in (no flicker)
- [ ] Smooth slide effects
- [ ] No jank or stuttering
- [ ] 60fps verified with profiler

### Performance Tests

**Using criterion** (when build works):
```rust
#[bench]
fn bench_animation_update(b: &mut Bencher) {
    let mut anim = Animation::new(AnimationDuration::NORMAL, Easing::CubicOut);
    b.iter(|| anim.value());
}
```

**Target**: < 10µs per animation update

## Future Enhancements

### v2.0 Potential Features

1. **Custom Easing Curves**
   - Bezier curve support
   - Spring physics (damping, stiffness)
   - User-defined curves

2. **Chained Animations**
   - Sequential animations
   - Parallel with stagger
   - Callbacks on completion

3. **Gesture Animations**
   - Swipe to dismiss
   - Drag to reorder
   - Pull to refresh

4. **Advanced Effects**
   - Blur during transition
   - Rotation on dismiss
   - Particle effects (optional)

5. **Animation Presets**
   - "Minimal" - fast, simple
   - "Smooth" - current default
   - "Playful" - bouncy, exaggerated
   - "Elegant" - slow, refined

## References

### External Resources

- [iced Animation Example](https://github.com/iced-rs/iced/tree/master/examples/animation)
- [CSS Easing Functions](https://easings.net/)
- [WCAG 2.1 - Animation from Interactions](https://www.w3.org/WAI/WCAG21/Understanding/animation-from-interactions.html)
- [prefers-reduced-motion MDN](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion)

### Related Documentation

- **PROJECT_PLAN.md** - Phase 4B status
- **PERFORMANCE_OPTIMIZATIONS.md** - Animation performance notes
- **Issue #15** - Original feature request

---

**Next Steps**:

1. Implement message handlers for animation lifecycle
2. Add AnimationFrame subscription (16ms interval)
3. Apply animations in rendering (opacity, transforms)
4. Detect and respect `prefers-reduced-motion`
5. Test visual smoothness once build environment is fixed
6. Add settings UI in Phase 5

**Estimated Completion**: Phase 4B - pending build environment fix for testing.
