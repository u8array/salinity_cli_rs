using System;
using System.Runtime.InteropServices;

[StructLayout(LayoutKind.Sequential)]
struct InputsC {
    public double na, ca, mg, k, sr, br, s, b, cl;
    public byte cl_present; // 0 = none, 1 = some
    public double f;
    public byte f_present;
    public double alk_dkh;
    public byte alk_dkh_present;
}

[StructLayout(LayoutKind.Sequential)]
struct AssumptionsC {
    public double temp;
    public double pressure_dbar;
    public double alkalinity; public byte alkalinity_present;
    public byte assume_borate;
    public double default_f_mg_l;
    public double ref_alk_dkh; public byte ref_alk_dkh_present;
    public double salinity_norm;
    public byte return_components;
    public double borate_fraction; public byte borate_fraction_present;
    public double alk_mg_per_meq; public byte alk_mg_per_meq_present;
    public byte rn_compat;
}

[StructLayout(LayoutKind.Sequential)]
struct CalculationSummaryC {
    public double sp, sa, density_kg_per_m3, sg_20_20, sg_25_25;
}

static class Native {
    [DllImport("salinity_rs", CallingConvention = CallingConvention.Cdecl)]
    public static extern int salinity_compute_summary_struct(in InputsC inputs, IntPtr assumptionsNullable, out CalculationSummaryC summary);

    [DllImport("salinity_rs", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr salinity_last_error();
}

class Program {
    static string ErrStr() => Marshal.PtrToStringAnsi(Native.salinity_last_error());

    static void Main() {
        var inputs = new InputsC {
            na = 10780.0, mg = 1290.0, ca = 420.0, k = 400.0, sr = 8.0,
            br = 65.0, s = 900.0, b = 4.4,
            cl = 0.0, cl_present = 0,
            f = 0.0, f_present = 0,
            alk_dkh = 8.0, alk_dkh_present = 1
        };

        if (Native.salinity_compute_summary_struct(in inputs, IntPtr.Zero, out var outSummary) != 0) {
            Console.WriteLine("Error: " + ErrStr());
            return;
        }
        Console.WriteLine($"SP={outSummary.sp:F4} SA={outSummary.sa:F4} rho={outSummary.density_kg_per_m3:F3} SG20/20={outSummary.sg_20_20:F5}");
    }
}
