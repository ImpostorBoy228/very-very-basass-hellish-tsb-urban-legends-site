#include "common.h"
#include <gmp.h>

void its_nsecs_str(char *buf, size_t len) {
    (void)len;
    load_finals("ITS/finals.all");
    build_spline();

    int ey, em, ed;
    double offset = compute_earliest_night(&ey, &em, &ed);
    double night_mjd = jdn(ey, em, ed) - 2400000.5;
    double night_dut1 = interpolate_dut1_spline(night_mjd);

    struct timespec now_ts;
    clock_gettime(CLOCK_REALTIME, &now_ts);
    double now_dut1 = interpolate_dut1_spline(mjd_from_unix(now_ts.tv_sec));

    struct timespec epoch_ts;
    epoch_ts.tv_sec = (time_t)EPOCH_UNIX;
    epoch_ts.tv_nsec = 0;

    mpz_t epoch_ns, now_ns, night_dut1_ns, now_dut1_ns, offset_ns;
    mpz_t epoch_start_ns, delta_ns;
    mpz_inits(epoch_ns, now_ns, night_dut1_ns, now_dut1_ns, offset_ns, epoch_start_ns, delta_ns, NULL);

    mpz_set_si(epoch_ns, epoch_ts.tv_sec);
    mpz_mul_ui(epoch_ns, epoch_ns, 1000000000ULL);
    mpz_add_ui(epoch_ns, epoch_ns, epoch_ts.tv_nsec);

    mpz_set_si(now_ns, now_ts.tv_sec);
    mpz_mul_ui(now_ns, now_ns, 1000000000ULL);
    mpz_add_ui(now_ns, now_ns, now_ts.tv_nsec);

    mpz_set_d(night_dut1_ns, night_dut1 * 1e9);
    mpz_set_d(now_dut1_ns, now_dut1 * 1e9);
    mpz_set_d(offset_ns, offset * 1e9);

    mpz_add(epoch_start_ns, epoch_ns, night_dut1_ns);
    mpz_add(epoch_start_ns, epoch_start_ns, offset_ns);

    mpz_add(delta_ns, now_ns, now_dut1_ns);
    mpz_sub(delta_ns, delta_ns, epoch_start_ns);

    mpz_get_str(buf, 10, delta_ns);

    mpz_clears(epoch_ns, now_ns, night_dut1_ns, now_dut1_ns, offset_ns, epoch_start_ns, delta_ns, NULL);
    free_eop();
}
