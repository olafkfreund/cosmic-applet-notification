// Animation system for notification transitions
//
// Provides smooth animations for notification appearance, dismissal, and UI transitions.
// Respects accessibility preferences for reduced motion.

use std::time::{Duration, Instant};

/// Animation duration in milliseconds
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnimationDuration(pub u64);

impl AnimationDuration {
    /// No animation (instant)
    pub const INSTANT: Self = Self(0);

    /// Fast animation (200ms) - for quick interactions
    pub const FAST: Self = Self(200);

    /// Normal animation (300ms) - default for most animations
    pub const NORMAL: Self = Self(300);

    /// Slow animation (500ms) - for emphasis
    pub const SLOW: Self = Self(500);

    /// Convert to Duration
    pub fn as_duration(self) -> Duration {
        Duration::from_millis(self.0)
    }

    /// Get duration in milliseconds
    pub fn as_millis(self) -> u64 {
        self.0
    }
}

/// Easing function for smooth animations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Easing {
    /// Linear interpolation (no easing)
    Linear,

    /// Ease in (slow start, fast end)
    EaseIn,

    /// Ease out (fast start, slow end)
    EaseOut,

    /// Ease in-out (slow start and end, fast middle)
    EaseInOut,

    /// Cubic ease in
    CubicIn,

    /// Cubic ease out
    CubicOut,

    /// Cubic ease in-out
    CubicInOut,

    /// Exponential ease out (smooth deceleration)
    ExpoOut,

    /// Bounce ease out (spring-like effect)
    BounceOut,
}

impl Easing {
    /// Apply easing function to a progress value (0.0 to 1.0)
    ///
    /// Returns a value from 0.0 to 1.0 with the easing curve applied.
    pub fn apply(self, progress: f32) -> f32 {
        let t = progress.clamp(0.0, 1.0);

        match self {
            Easing::Linear => t,

            Easing::EaseIn => t * t,

            Easing::EaseOut => t * (2.0 - t),

            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }

            Easing::CubicIn => t * t * t,

            Easing::CubicOut => {
                let t1 = t - 1.0;
                t1 * t1 * t1 + 1.0
            }

            Easing::CubicInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let t1 = 2.0 * t - 2.0;
                    1.0 + t1 * t1 * t1 / 2.0
                }
            }

            Easing::ExpoOut => {
                if t >= 1.0 {
                    1.0
                } else {
                    1.0 - 2.0_f32.powf(-10.0 * t)
                }
            }

            Easing::BounceOut => {
                if t < 1.0 / 2.75 {
                    7.5625 * t * t
                } else if t < 2.0 / 2.75 {
                    let t1 = t - 1.5 / 2.75;
                    7.5625 * t1 * t1 + 0.75
                } else if t < 2.5 / 2.75 {
                    let t1 = t - 2.25 / 2.75;
                    7.5625 * t1 * t1 + 0.9375
                } else {
                    let t1 = t - 2.625 / 2.75;
                    7.5625 * t1 * t1 + 0.984375
                }
            }
        }
    }
}

/// Animation state for a single animatable value
#[derive(Debug, Clone)]
pub struct Animation {
    /// Start time of the animation
    start_time: Instant,

    /// Duration of the animation
    duration: AnimationDuration,

    /// Easing function
    easing: Easing,

    /// Start value (0.0 to 1.0)
    start_value: f32,

    /// End value (0.0 to 1.0)
    end_value: f32,

    /// Whether animation is complete
    completed: bool,
}

impl Animation {
    /// Create a new animation
    pub fn new(duration: AnimationDuration, easing: Easing) -> Self {
        Self {
            start_time: Instant::now(),
            duration,
            easing,
            start_value: 0.0,
            end_value: 1.0,
            completed: duration.as_millis() == 0,
        }
    }

    /// Create an animation from one value to another
    pub fn from_to(duration: AnimationDuration, easing: Easing, start: f32, end: f32) -> Self {
        Self {
            start_time: Instant::now(),
            duration,
            easing,
            start_value: start,
            end_value: end,
            completed: duration.as_millis() == 0,
        }
    }

    /// Create a fade-in animation (0.0 to 1.0)
    pub fn fade_in(duration: AnimationDuration) -> Self {
        Self::from_to(duration, Easing::EaseOut, 0.0, 1.0)
    }

    /// Create a fade-out animation (1.0 to 0.0)
    pub fn fade_out(duration: AnimationDuration) -> Self {
        Self::from_to(duration, Easing::EaseIn, 1.0, 0.0)
    }

    /// Create a slide-in animation (for translation)
    pub fn slide_in(duration: AnimationDuration) -> Self {
        Self::from_to(duration, Easing::CubicOut, 1.0, 0.0)
    }

    /// Create a slide-out animation
    pub fn slide_out(duration: AnimationDuration) -> Self {
        Self::from_to(duration, Easing::CubicIn, 0.0, 1.0)
    }

    /// Get current animation value (0.0 to 1.0)
    pub fn value(&self) -> f32 {
        if self.completed {
            return self.end_value;
        }

        let elapsed = self.start_time.elapsed();
        let duration_ms = self.duration.as_millis();

        if duration_ms == 0 {
            return self.end_value;
        }

        let progress = elapsed.as_millis() as f32 / duration_ms as f32;

        if progress >= 1.0 {
            self.end_value
        } else {
            let eased_progress = self.easing.apply(progress);
            self.start_value + (self.end_value - self.start_value) * eased_progress
        }
    }

    /// Check if animation is complete
    pub fn is_complete(&self) -> bool {
        if self.completed {
            return true;
        }

        let elapsed = self.start_time.elapsed();
        elapsed >= self.duration.as_duration()
    }

    /// Mark animation as complete
    pub fn complete(&mut self) {
        self.completed = true;
    }

    /// Reset animation to start again
    pub fn reset(&mut self) {
        self.start_time = Instant::now();
        self.completed = false;
    }

    /// Get progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        if self.completed {
            return 1.0;
        }

        let elapsed = self.start_time.elapsed();
        let duration_ms = self.duration.as_millis();

        if duration_ms == 0 {
            return 1.0;
        }

        (elapsed.as_millis() as f32 / duration_ms as f32).min(1.0)
    }
}

/// Animation type for notifications
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NotificationAnimationType {
    /// Notification is appearing (slide-in + fade-in)
    Appearing,

    /// Notification is being dismissed (fade-out)
    Dismissing,

    /// Notification is idle (no animation)
    Idle,
}

/// Animation state for a single notification
#[derive(Debug, Clone)]
pub struct NotificationAnimation {
    /// Notification ID
    pub notification_id: u32,

    /// Current animation type
    pub animation_type: NotificationAnimationType,

    /// Opacity animation (0.0 = invisible, 1.0 = fully visible)
    pub opacity: Animation,

    /// Translation animation (for slide effects)
    /// Value represents offset in pixels
    pub translation_y: Animation,

    /// Scale animation (for zoom effects)
    pub scale: Animation,
}

impl NotificationAnimation {
    /// Create a new appearing animation
    pub fn appearing(notification_id: u32, duration: AnimationDuration) -> Self {
        Self {
            notification_id,
            animation_type: NotificationAnimationType::Appearing,
            opacity: Animation::fade_in(duration),
            translation_y: Animation::slide_in(duration),
            scale: Animation::from_to(duration, Easing::CubicOut, 0.95, 1.0),
        }
    }

    /// Create a new dismissing animation
    pub fn dismissing(notification_id: u32, duration: AnimationDuration) -> Self {
        Self {
            notification_id,
            animation_type: NotificationAnimationType::Dismissing,
            opacity: Animation::fade_out(duration),
            translation_y: Animation::from_to(duration, Easing::EaseIn, 0.0, -50.0),
            scale: Animation::from_to(duration, Easing::EaseIn, 1.0, 0.95),
        }
    }

    /// Create an idle (non-animated) state
    pub fn idle(notification_id: u32) -> Self {
        Self {
            notification_id,
            animation_type: NotificationAnimationType::Idle,
            opacity: Animation::from_to(AnimationDuration::INSTANT, Easing::Linear, 1.0, 1.0),
            translation_y: Animation::from_to(AnimationDuration::INSTANT, Easing::Linear, 0.0, 0.0),
            scale: Animation::from_to(AnimationDuration::INSTANT, Easing::Linear, 1.0, 1.0),
        }
    }

    /// Check if all animations are complete
    pub fn is_complete(&self) -> bool {
        self.opacity.is_complete() && self.translation_y.is_complete() && self.scale.is_complete()
    }

    /// Get current opacity value
    pub fn opacity_value(&self) -> f32 {
        self.opacity.value()
    }

    /// Get current translation Y value (in pixels)
    pub fn translation_y_value(&self) -> f32 {
        self.translation_y.value()
    }

    /// Get current scale value
    pub fn scale_value(&self) -> f32 {
        self.scale.value()
    }
}

/// Popup animation state
#[derive(Debug, Clone)]
pub struct PopupAnimation {
    /// Animation type
    pub is_opening: bool,

    /// Opacity animation
    pub opacity: Animation,

    /// Scale animation
    pub scale: Animation,
}

impl PopupAnimation {
    /// Create opening animation
    pub fn opening(duration: AnimationDuration) -> Self {
        Self {
            is_opening: true,
            opacity: Animation::fade_in(duration),
            scale: Animation::from_to(duration, Easing::CubicOut, 0.95, 1.0),
        }
    }

    /// Create closing animation
    pub fn closing(duration: AnimationDuration) -> Self {
        Self {
            is_opening: false,
            opacity: Animation::fade_out(duration),
            scale: Animation::from_to(duration, Easing::CubicIn, 1.0, 0.95),
        }
    }

    /// Check if animation is complete
    pub fn is_complete(&self) -> bool {
        self.opacity.is_complete() && self.scale.is_complete()
    }

    /// Get current opacity
    pub fn opacity_value(&self) -> f32 {
        self.opacity.value()
    }

    /// Get current scale
    pub fn scale_value(&self) -> f32 {
        self.scale.value()
    }
}

/// Progress indicator for timed notifications
#[derive(Debug, Clone)]
pub struct ProgressIndicator {
    /// Notification ID
    pub notification_id: u32,

    /// Total duration in seconds
    pub total_duration: i64,

    /// Start time
    pub start_time: Instant,
}

impl ProgressIndicator {
    /// Create a new progress indicator
    pub fn new(notification_id: u32, duration_seconds: i64) -> Self {
        Self {
            notification_id,
            total_duration: duration_seconds,
            start_time: Instant::now(),
        }
    }

    /// Get progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        if self.total_duration <= 0 {
            return 1.0;
        }

        let elapsed = self.start_time.elapsed().as_secs() as f32;
        let total = self.total_duration as f32;

        (elapsed / total).min(1.0)
    }

    /// Get remaining time in seconds
    pub fn remaining_seconds(&self) -> i64 {
        let elapsed = self.start_time.elapsed().as_secs() as i64;
        (self.total_duration - elapsed).max(0)
    }

    /// Check if time is up
    pub fn is_expired(&self) -> bool {
        self.start_time.elapsed().as_secs() as i64 >= self.total_duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_easing_linear() {
        assert_eq!(Easing::Linear.apply(0.0), 0.0);
        assert_eq!(Easing::Linear.apply(0.5), 0.5);
        assert_eq!(Easing::Linear.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_in() {
        let result = Easing::EaseIn.apply(0.5);
        assert!(result < 0.5); // Should be slower at start
    }

    #[test]
    fn test_easing_ease_out() {
        let result = Easing::EaseOut.apply(0.5);
        assert!(result > 0.5); // Should be faster at start
    }

    #[test]
    fn test_easing_bounds() {
        for easing in [
            Easing::Linear,
            Easing::EaseIn,
            Easing::EaseOut,
            Easing::EaseInOut,
            Easing::CubicIn,
            Easing::CubicOut,
            Easing::CubicInOut,
            Easing::ExpoOut,
            Easing::BounceOut,
        ] {
            assert_eq!(easing.apply(0.0), 0.0);
            assert!((easing.apply(1.0) - 1.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_animation_instant() {
        let anim = Animation::new(AnimationDuration::INSTANT, Easing::Linear);
        assert!(anim.is_complete());
        assert_eq!(anim.value(), 1.0);
    }

    #[test]
    fn test_animation_fade_in() {
        let anim = Animation::fade_in(AnimationDuration::FAST);
        let initial = anim.value();
        assert!(initial <= 0.1); // Should start near 0
    }

    #[test]
    fn test_animation_fade_out() {
        let anim = Animation::fade_out(AnimationDuration::FAST);
        let initial = anim.value();
        assert!(initial >= 0.9); // Should start near 1
    }

    #[test]
    fn test_animation_progress() {
        let mut anim = Animation::new(AnimationDuration(100), Easing::Linear);

        // Initial progress
        assert!(anim.progress() < 0.1);

        // Wait and check progress increased
        thread::sleep(Duration::from_millis(50));
        let progress = anim.progress();
        assert!(progress > 0.4 && progress < 0.7);

        // Mark complete
        anim.complete();
        assert_eq!(anim.progress(), 1.0);
    }

    #[test]
    fn test_notification_animation_appearing() {
        let anim = NotificationAnimation::appearing(1, AnimationDuration::NORMAL);
        assert_eq!(anim.animation_type, NotificationAnimationType::Appearing);
        assert!(anim.opacity_value() <= 0.1);
    }

    #[test]
    fn test_notification_animation_dismissing() {
        let anim = NotificationAnimation::dismissing(1, AnimationDuration::NORMAL);
        assert_eq!(anim.animation_type, NotificationAnimationType::Dismissing);
        assert!(anim.opacity_value() >= 0.9);
    }

    #[test]
    fn test_notification_animation_idle() {
        let anim = NotificationAnimation::idle(1);
        assert_eq!(anim.animation_type, NotificationAnimationType::Idle);
        assert_eq!(anim.opacity_value(), 1.0);
        assert!(anim.is_complete());
    }

    #[test]
    fn test_popup_animation() {
        let opening = PopupAnimation::opening(AnimationDuration::NORMAL);
        assert!(opening.is_opening);
        assert!(opening.opacity_value() <= 0.1);

        let closing = PopupAnimation::closing(AnimationDuration::NORMAL);
        assert!(!closing.is_opening);
        assert!(closing.opacity_value() >= 0.9);
    }

    #[test]
    fn test_progress_indicator() {
        let indicator = ProgressIndicator::new(1, 10);

        assert!(indicator.progress() < 0.1);
        assert_eq!(indicator.remaining_seconds(), 10);
        assert!(!indicator.is_expired());
    }

    #[test]
    fn test_progress_indicator_zero_duration() {
        let indicator = ProgressIndicator::new(1, 0);
        assert_eq!(indicator.progress(), 1.0);
        assert!(indicator.is_expired());
    }
}
