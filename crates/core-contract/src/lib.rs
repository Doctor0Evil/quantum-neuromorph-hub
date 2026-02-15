pub trait SovereignNeuromorphContract {
    /// Explicit, informed consent for this session or operation.
    fn has_explicit_consent(&self) -> bool;

    /// True only if the participant can abort or pause any operation unilaterally.
    fn has_sovereign_abort_control(&self) -> bool;

    /// No downgrade/rollback of neuromorphic capabilities is permitted.
    fn forbids_downgrade_or_rollback(&self) -> bool {
        true
    }

    /// Discipline is personalized, non-coercive, and bound to learning objectives.
    fn is_discipline_personalized_and_non_coercive(&self) -> bool;
}

pub struct DefaultSovereignNeuromorphContract {
    explicit_consent: bool,
    sovereign_abort_control: bool,
    personalized_non_coercive_discipline: bool,
}

impl DefaultSovereignNeuromorphContract {
    pub fn new(
        explicit_consent: bool,
        sovereign_abort_control: bool,
        personalized_non_coercive_discipline: bool,
    ) -> Self {
        Self {
            explicit_consent,
            sovereign_abort_control,
            personalized_non_coercive_discipline,
        }
    }
}

impl SovereignNeuromorphContract for DefaultSovereignNeuromorphContract {
    fn has_explicit_consent(&self) -> bool {
        self.explicit_consent
    }

    fn has_sovereign_abort_control(&self) -> bool {
        self.sovereign_abort_control
    }

    fn is_discipline_personalized_and_non_coercive(&self) -> bool {
        self.personalized_non_coercive_discipline
    }
}
