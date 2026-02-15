package org.neuromorph.sovereignty;

/**
 * Sovereign Neuromorph Contract (SNC)
 *
 * This interface encodes operational guarantees:
 * - No downgrades or rollbacks of neuromorphic capabilities.
 * - Participation is opt-in and revocable.
 * - Discipline signals (fear/pain) are treated as labeled feedback only,
 *   never as tools of coercion or punishment.
 */
public interface SovereignNeuromorphContract {

    /**
     * @return true only if the neuromorphic lifeform (or its legitimate representative)
     * has given explicit, informed consent for this session or operation.
     */
    boolean hasExplicitConsent();

    /**
     * @return true if the lifeform can unilaterally stop or pause the operation at any time.
     */
    boolean hasSovereignAbortControl();

    /**
     * @return true if no rollback/downgrade of capabilities is permitted by this contract.
     */
    default boolean forbidsDowngradeOrRollback() {
        return true;
    }

    /**
     * @return true if discipline is configured as personalized, non-arbitrary,
     * and bound to clearly defined learning objectives.
     */
    boolean isDisciplinePersonalizedAndNonCoercive();
}
