use crate::*;

/// Inline helpers from <infiniband/mlx5dv.h> that operate through function
/// pointers returned by mlx5dv_qp_ex_from_ibv_qp_ex().
#[inline]
pub unsafe fn mlx5dv_wr_set_dc_addr(
    mqp: *mut mlx5dv_qp_ex,
    ah: *mut ibv_ah,
    remote_dctn: u32,
    remote_dc_key: u64,
) {
    (*mqp).wr_set_dc_addr.unwrap_unchecked()(mqp, ah, remote_dctn, remote_dc_key);
}

#[inline]
pub unsafe fn mlx5dv_wr_set_dc_addr_stream(
    mqp: *mut mlx5dv_qp_ex,
    ah: *mut ibv_ah,
    remote_dctn: u32,
    remote_dc_key: u64,
    stream_id: u16,
) {
    (*mqp).wr_set_dc_addr_stream.unwrap_unchecked()(mqp, ah, remote_dctn, remote_dc_key, stream_id);
}

#[inline]
pub unsafe fn mlx5dv_wr_mr_interleaved(
    mqp: *mut mlx5dv_qp_ex,
    mkey: *mut mlx5dv_mkey,
    access_flags: u32,
    repeat_count: u32,
    num_interleaved: u16,
    data: *mut mlx5dv_mr_interleaved,
) {
    (*mqp).wr_mr_interleaved.unwrap_unchecked()(
        mqp,
        mkey,
        access_flags,
        repeat_count,
        num_interleaved,
        data,
    );
}

#[inline]
pub unsafe fn mlx5dv_wr_mr_list(
    mqp: *mut mlx5dv_qp_ex,
    mkey: *mut mlx5dv_mkey,
    access_flags: u32,
    num_sges: u16,
    sge: *mut ibv_sge,
) {
    (*mqp).wr_mr_list.unwrap_unchecked()(mqp, mkey, access_flags, num_sges, sge);
}

#[inline]
pub unsafe fn mlx5dv_wr_mkey_configure(
    mqp: *mut mlx5dv_qp_ex,
    mkey: *mut mlx5dv_mkey,
    num_setters: u8,
    attr: *mut mlx5dv_mkey_conf_attr,
) {
    (*mqp).wr_mkey_configure.unwrap_unchecked()(mqp, mkey, num_setters, attr);
}

#[inline]
pub unsafe fn mlx5dv_wr_set_mkey_access_flags(mqp: *mut mlx5dv_qp_ex, access_flags: u32) {
    (*mqp).wr_set_mkey_access_flags.unwrap_unchecked()(mqp, access_flags);
}

#[inline]
pub unsafe fn mlx5dv_wr_set_mkey_layout_list(
    mqp: *mut mlx5dv_qp_ex,
    num_sges: u16,
    sge: *const ibv_sge,
) {
    (*mqp).wr_set_mkey_layout_list.unwrap_unchecked()(mqp, num_sges, sge);
}

#[inline]
pub unsafe fn mlx5dv_wr_set_mkey_layout_interleaved(
    mqp: *mut mlx5dv_qp_ex,
    repeat_count: u32,
    num_interleaved: u16,
    data: *const mlx5dv_mr_interleaved,
) {
    (*mqp).wr_set_mkey_layout_interleaved.unwrap_unchecked()(
        mqp,
        repeat_count,
        num_interleaved,
        data,
    );
}

#[inline]
pub unsafe fn mlx5dv_wr_set_mkey_sig_block(
    mqp: *mut mlx5dv_qp_ex,
    attr: *const mlx5dv_sig_block_attr,
) {
    (*mqp).wr_set_mkey_sig_block.unwrap_unchecked()(mqp, attr);
}

#[inline]
pub unsafe fn mlx5dv_wr_set_mkey_crypto(mqp: *mut mlx5dv_qp_ex, attr: *const mlx5dv_crypto_attr) {
    (*mqp).wr_set_mkey_crypto.unwrap_unchecked()(mqp, attr);
}

#[inline]
pub unsafe fn mlx5dv_wr_memcpy(
    mqp: *mut mlx5dv_qp_ex,
    dest_lkey: u32,
    dest_addr: u64,
    src_lkey: u32,
    src_addr: u64,
    length: usize,
) {
    (*mqp).wr_memcpy.unwrap_unchecked()(mqp, dest_lkey, dest_addr, src_lkey, src_addr, length);
}

#[inline]
pub unsafe fn mlx5dv_wr_raw_wqe(mqp: *mut mlx5dv_qp_ex, wqe: *const c_void) {
    (*mqp).wr_raw_wqe.unwrap_unchecked()(mqp, wqe);
}
