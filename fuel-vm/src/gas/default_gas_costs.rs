use super::*;

pub const GIT: &str = "1a660f59b892e7816da01c288c86696c9d7d7a36";
pub fn default_gas_costs() -> GasCostsValues {
    GasCostsValues {
        add: 1,
        addi: 1,
        aloc: 1,
        and: 1,
        andi: 1,
        bal: 155,
        bhei: 1,
        bhsh: 1,
        burn: 35,
        cb: 2,
        cfei: 1,
        cfsi: 1,
        croo: 163,
        div: 1,
        divi: 1,
        ecr: 1601,
        eq: 1,
        exp: 1,
        expi: 1,
        flag: 1,
        gm: 1,
        gt: 1,
        gtf: 1,
        ji: 1,
        jmp: 1,
        jne: 1,
        jnei: 1,
        jnzi: 1,
        k256: 18,
        lb: 1,
        log: 40,
        lt: 1,
        lw: 1,
        mcpi: 3,
        mint: 34,
        mlog: 1,
        mod_op: 1,
        modi: 1,
        move_op: 1,
        movi: 1,
        mroo: 2,
        mul: 1,
        muli: 1,
        noop: 1,
        not: 1,
        or: 1,
        ori: 1,
        ret: 1,
        rvrt: 1,
        s256: 5,
        sb: 1,
        scwq: 10,
        sll: 1,
        slli: 1,
        srl: 1,
        srli: 1,
        srw: 160,
        sub: 1,
        subi: 1,
        sw: 1,
        sww: 80,
        swwq: 69,
        time: 1,
        tr: 113,
        tro: 94,
        xor: 1,
        xori: 1,
        call: DependentCost {
            base: 311,
            dep_per_unit: 14,
        },
        ccp: DependentCost {
            base: 155,
            dep_per_unit: 15,
        },
        csiz: DependentCost {
            base: 148,
            dep_per_unit: 16,
        },
        ldc: DependentCost {
            base: 156,
            dep_per_unit: 14,
        },
        logd: DependentCost {
            base: 47,
            dep_per_unit: 21,
        },
        mcl: DependentCost {
            base: 1,
            dep_per_unit: 2539,
        },
        mcli: DependentCost {
            base: 1,
            dep_per_unit: 2610,
        },
        mcp: DependentCost {
            base: 1,
            dep_per_unit: 1377,
        },
        meq: DependentCost {
            base: 1,
            dep_per_unit: 2802,
        },
        retd: DependentCost {
            base: 64,
            dep_per_unit: 21,
        },
        smo: DependentCost {
            base: 79,
            dep_per_unit: 14,
        },
        srwq: DependentCost {
            base: 424,
            dep_per_unit: 6,
        },
    }
}