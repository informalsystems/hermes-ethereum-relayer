use alloy::sol;

sol!(
    struct ClientState {
        string chainId;
        TrustThreshold trustLevel;
        Height latestHeight;
        uint32 trustingPeriod;
        uint32 unbondingPeriod;
        bool isFrozen;
        SupportedZkAlgorithm zkAlgorithm;
    }

    struct TrustThreshold {
        uint8 numerator;
        uint8 denominator;
    }

    enum SupportedZkAlgorithm {
        Groth16,
        Plonk
    }

    struct Height {
        uint32 revisionNumber;
        uint32 revisionHeight;
    }
);
