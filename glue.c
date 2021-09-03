#include <uk/sched.h>
#include <uk/plat/lcpu.h>
#include <uk/schedcoop.h>
#include <uk/plat/memory.h>
#include <uk/plat/time.h>
#include "sched.h"
#ifdef __cplusplus
            extern "C"{
#endif

uint64_t uk_schedcoop_init(struct uk_alloc *a){
    struct schedcoop_private *prv = NULL;
    struct uk_sched *sched = NULL;

	rust_init_sched();

    uk_pr_info("Initializing cooperative scheduler\n");

    sched = uk_sched_create(a, 0);
    if (sched == NULL)
        return NULL;

    ukplat_ctx_callbacks_init(&sched->plat_ctx_cbs, ukplat_ctx_sw);

    uk_sched_idle_init(sched, NULL, idle_thread_fn);

    uk_sched_init(sched,
                    schedcoop_yield,
                    schedcoop_thread_add,
                    schedcoop_thread_remove,
                    schedcoop_thread_blocked,
                    schedcoop_thread_woken,
                    NULL, NULL, NULL, NULL);
	uk_pr_info("WOLO%p\n", sched);
    return (uint64_t) sched;
}


void schedcoop_thread_woken(struct uk_sched *s, struct uk_thread *t) {

}

void schedcoop_schedule(struct uk_sched *s) {

}

int schedcoop_thread_add(struct uk_sched *s, struct uk_thread *t, const uk_thread_attr_t *attr __unused){

}

void schedcoop_thread_remove(struct uk_sched *s, struct uk_thread *t) {

}

void schedcoop_yield(struct uk_sched *s) {

}

void idle_thread_fn(void *unused __unused) {

}

void schedcoop_thread_blocked(struct uk_sched *s, struct uk_thread *t) {
    UK_ASSERT(ukplat_lcpu_irqs_disabled());
}

#ifdef __cplusplus
}
#endif