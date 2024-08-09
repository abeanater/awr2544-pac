#[doc = "Register `DSS_TPCC_A_PARITY_CTRL` reader"]
pub type R = crate::R<DssTpccAParityCtrlSpec>;
#[doc = "Register `DSS_TPCC_A_PARITY_CTRL` writer"]
pub type W = crate::W<DssTpccAParityCtrlSpec>;
#[doc = "Field `parity_en` reader - 0:0\\]
Enable Parity for TPCC. Write 0x1 : Parity is enabled on PARAM memory"]
pub type ParityEnR = crate::BitReader;
#[doc = "Field `parity_en` writer - 0:0\\]
Enable Parity for TPCC. Write 0x1 : Parity is enabled on PARAM memory"]
pub type ParityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `parity_testen` reader - 1:1\\]
Enable Parity Test for TPCC. Write 0x1 : Parity Test is enabled on PARAM memory"]
pub type ParityTestenR = crate::BitReader;
#[doc = "Field `parity_testen` writer - 1:1\\]
Enable Parity Test for TPCC. Write 0x1 : Parity Test is enabled on PARAM memory"]
pub type ParityTestenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `parity_err_clr` reader - 2:2\\]
Write pulse bit field: Write 0x1 to clear the Parit Error status for TPCC"]
pub type ParityErrClrR = crate::BitReader;
#[doc = "Field `parity_err_clr` writer - 2:2\\]
Write pulse bit field: Write 0x1 to clear the Parit Error status for TPCC"]
pub type ParityErrClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable Parity for TPCC. Write 0x1 : Parity is enabled on PARAM memory"]
    #[inline(always)]
    pub fn parity_en(&self) -> ParityEnR {
        ParityEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable Parity Test for TPCC. Write 0x1 : Parity Test is enabled on PARAM memory"]
    #[inline(always)]
    pub fn parity_testen(&self) -> ParityTestenR {
        ParityTestenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Write pulse bit field: Write 0x1 to clear the Parit Error status for TPCC"]
    #[inline(always)]
    pub fn parity_err_clr(&self) -> ParityErrClrR {
        ParityErrClrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable Parity for TPCC. Write 0x1 : Parity is enabled on PARAM memory"]
    #[inline(always)]
    #[must_use]
    pub fn parity_en(&mut self) -> ParityEnW<DssTpccAParityCtrlSpec> {
        ParityEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable Parity Test for TPCC. Write 0x1 : Parity Test is enabled on PARAM memory"]
    #[inline(always)]
    #[must_use]
    pub fn parity_testen(&mut self) -> ParityTestenW<DssTpccAParityCtrlSpec> {
        ParityTestenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Write pulse bit field: Write 0x1 to clear the Parit Error status for TPCC"]
    #[inline(always)]
    #[must_use]
    pub fn parity_err_clr(&mut self) -> ParityErrClrW<DssTpccAParityCtrlSpec> {
        ParityErrClrW::new(self, 2)
    }
}
#[doc = "DSS_TPCC_A_PARITY_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tpcc_a_parity_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tpcc_a_parity_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssTpccAParityCtrlSpec;
impl crate::RegisterSpec for DssTpccAParityCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_tpcc_a_parity_ctrl::R`](R) reader structure"]
impl crate::Readable for DssTpccAParityCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_tpcc_a_parity_ctrl::W`](W) writer structure"]
impl crate::Writable for DssTpccAParityCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_TPCC_A_PARITY_CTRL to value 0"]
impl crate::Resettable for DssTpccAParityCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
