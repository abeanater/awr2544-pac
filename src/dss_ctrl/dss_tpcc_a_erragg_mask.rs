#[doc = "Register `DSS_TPCC_A_ERRAGG_MASK` reader"]
pub type R = crate::R<DssTpccAErraggMaskSpec>;
#[doc = "Register `DSS_TPCC_A_ERRAGG_MASK` writer"]
pub type W = crate::W<DssTpccAErraggMaskSpec>;
#[doc = "Field `tpcc_a_errint` reader - 0:0\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TpccAErrintR = crate::BitReader;
#[doc = "Field `tpcc_a_errint` writer - 0:0\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TpccAErrintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_a_mpint` reader - 1:1\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TpccAMpintR = crate::BitReader;
#[doc = "Field `tpcc_a_mpint` writer - 1:1\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TpccAMpintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_a0_err` reader - 2:2\\]
Mask Interrupt from DSS_TPTC_A0 to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TptcA0ErrR = crate::BitReader;
#[doc = "Field `tptc_a0_err` writer - 2:2\\]
Mask Interrupt from DSS_TPTC_A0 to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TptcA0ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_a1_err` reader - 3:3\\]
Mask Interrupt from DSS_TPTC_A1 to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TptcA1ErrR = crate::BitReader;
#[doc = "Field `tptc_a1_err` writer - 3:3\\]
Mask Interrupt from DSS_TPTC_A1 to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TptcA1ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_a_parity_err` reader - 8:8\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TpccAParityErrR = crate::BitReader;
#[doc = "Field `tpcc_a_parity_err` writer - 8:8\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TpccAParityErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_a_write_access_error` reader - 16:16\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TpccAWriteAccessErrorR = crate::BitReader;
#[doc = "Field `tpcc_a_write_access_error` writer - 16:16\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TpccAWriteAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_a0_write_access_error` reader - 17:17\\]
Mask Interrupt from DSS_TPTC_A0 to aggregated Interrupt DSS_TPTC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TptcA0WriteAccessErrorR = crate::BitReader;
#[doc = "Field `tptc_a0_write_access_error` writer - 17:17\\]
Mask Interrupt from DSS_TPTC_A0 to aggregated Interrupt DSS_TPTC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TptcA0WriteAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_a1_write_access_error` reader - 18:18\\]
Mask Interrupt from DSS_TPTC_A1 to aggregated Interrupt DSS_TPTC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TptcA1WriteAccessErrorR = crate::BitReader;
#[doc = "Field `tptc_a1_write_access_error` writer - 18:18\\]
Mask Interrupt from DSS_TPTC_A1 to aggregated Interrupt DSS_TPTC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TptcA1WriteAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_a_read_access_error` reader - 24:24\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TpccAReadAccessErrorR = crate::BitReader;
#[doc = "Field `tpcc_a_read_access_error` writer - 24:24\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TpccAReadAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_a0_read_access_error` reader - 25:25\\]
Mask Interrupt from DSS_TPTC_A0 to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TptcA0ReadAccessErrorR = crate::BitReader;
#[doc = "Field `tptc_a0_read_access_error` writer - 25:25\\]
Mask Interrupt from DSS_TPTC_A0 to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TptcA0ReadAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tptc_a1_read_access_error` reader - 26:26\\]
Mask Interrupt from DSS_TPTC_A1 to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TptcA1ReadAccessErrorR = crate::BitReader;
#[doc = "Field `tptc_a1_read_access_error` writer - 26:26\\]
Mask Interrupt from DSS_TPTC_A1 to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type TptcA1ReadAccessErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn tpcc_a_errint(&self) -> TpccAErrintR {
        TpccAErrintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn tpcc_a_mpint(&self) -> TpccAMpintR {
        TpccAMpintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Mask Interrupt from DSS_TPTC_A0 to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn tptc_a0_err(&self) -> TptcA0ErrR {
        TptcA0ErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Mask Interrupt from DSS_TPTC_A1 to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn tptc_a1_err(&self) -> TptcA1ErrR {
        TptcA1ErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn tpcc_a_parity_err(&self) -> TpccAParityErrR {
        TpccAParityErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn tpcc_a_write_access_error(&self) -> TpccAWriteAccessErrorR {
        TpccAWriteAccessErrorR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Mask Interrupt from DSS_TPTC_A0 to aggregated Interrupt DSS_TPTC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn tptc_a0_write_access_error(&self) -> TptcA0WriteAccessErrorR {
        TptcA0WriteAccessErrorR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Mask Interrupt from DSS_TPTC_A1 to aggregated Interrupt DSS_TPTC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn tptc_a1_write_access_error(&self) -> TptcA1WriteAccessErrorR {
        TptcA1WriteAccessErrorR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn tpcc_a_read_access_error(&self) -> TpccAReadAccessErrorR {
        TpccAReadAccessErrorR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Mask Interrupt from DSS_TPTC_A0 to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn tptc_a0_read_access_error(&self) -> TptcA0ReadAccessErrorR {
        TptcA0ReadAccessErrorR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Mask Interrupt from DSS_TPTC_A1 to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn tptc_a1_read_access_error(&self) -> TptcA1ReadAccessErrorR {
        TptcA1ReadAccessErrorR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_errint(&mut self) -> TpccAErrintW<DssTpccAErraggMaskSpec> {
        TpccAErrintW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_mpint(&mut self) -> TpccAMpintW<DssTpccAErraggMaskSpec> {
        TpccAMpintW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Mask Interrupt from DSS_TPTC_A0 to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0_err(&mut self) -> TptcA0ErrW<DssTpccAErraggMaskSpec> {
        TptcA0ErrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Mask Interrupt from DSS_TPTC_A1 to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a1_err(&mut self) -> TptcA1ErrW<DssTpccAErraggMaskSpec> {
        TptcA1ErrW::new(self, 3)
    }
    #[doc = "Bit 8 - 8:8\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_parity_err(&mut self) -> TpccAParityErrW<DssTpccAErraggMaskSpec> {
        TpccAParityErrW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_write_access_error(&mut self) -> TpccAWriteAccessErrorW<DssTpccAErraggMaskSpec> {
        TpccAWriteAccessErrorW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Mask Interrupt from DSS_TPTC_A0 to aggregated Interrupt DSS_TPTC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0_write_access_error(
        &mut self,
    ) -> TptcA0WriteAccessErrorW<DssTpccAErraggMaskSpec> {
        TptcA0WriteAccessErrorW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Mask Interrupt from DSS_TPTC_A1 to aggregated Interrupt DSS_TPTC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a1_write_access_error(
        &mut self,
    ) -> TptcA1WriteAccessErrorW<DssTpccAErraggMaskSpec> {
        TptcA1WriteAccessErrorW::new(self, 18)
    }
    #[doc = "Bit 24 - 24:24\\]
Mask Interrupt from DSS_TPCC_A to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_read_access_error(&mut self) -> TpccAReadAccessErrorW<DssTpccAErraggMaskSpec> {
        TpccAReadAccessErrorW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Mask Interrupt from DSS_TPTC_A0 to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a0_read_access_error(&mut self) -> TptcA0ReadAccessErrorW<DssTpccAErraggMaskSpec> {
        TptcA0ReadAccessErrorW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Mask Interrupt from DSS_TPTC_A1 to aggregated Interrupt DSS_TPCC_A_INTAGG 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn tptc_a1_read_access_error(&mut self) -> TptcA1ReadAccessErrorW<DssTpccAErraggMaskSpec> {
        TptcA1ReadAccessErrorW::new(self, 26)
    }
}
#[doc = "DSS_TPCC_A_ERRAGG_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tpcc_a_erragg_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tpcc_a_erragg_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssTpccAErraggMaskSpec;
impl crate::RegisterSpec for DssTpccAErraggMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_tpcc_a_erragg_mask::R`](R) reader structure"]
impl crate::Readable for DssTpccAErraggMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_tpcc_a_erragg_mask::W`](W) writer structure"]
impl crate::Writable for DssTpccAErraggMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_TPCC_A_ERRAGG_MASK to value 0"]
impl crate::Resettable for DssTpccAErraggMaskSpec {
    const RESET_VALUE: u32 = 0;
}
