#[doc = "Register `DSS_TPCC_MEMINIT_STATUS` reader"]
pub type R = crate::R<DssTpccMeminitStatusSpec>;
#[doc = "Register `DSS_TPCC_MEMINIT_STATUS` writer"]
pub type W = crate::W<DssTpccMeminitStatusSpec>;
#[doc = "Field `tpcc_a_meminit_status` reader - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
pub type TpccAMeminitStatusR = crate::BitReader;
#[doc = "Field `tpcc_a_meminit_status` writer - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
pub type TpccAMeminitStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_b_meminit_status` reader - 1:1\\]
RESERVED: Dont Use"]
pub type TpccBMeminitStatusR = crate::BitReader;
#[doc = "Field `tpcc_b_meminit_status` writer - 1:1\\]
RESERVED: Dont Use"]
pub type TpccBMeminitStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tpcc_c_meminit_status` reader - 2:2\\]
RESERVED: Dont Use"]
pub type TpccCMeminitStatusR = crate::BitReader;
#[doc = "Field `tpcc_c_meminit_status` writer - 2:2\\]
RESERVED: Dont Use"]
pub type TpccCMeminitStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
    #[inline(always)]
    pub fn tpcc_a_meminit_status(&self) -> TpccAMeminitStatusR {
        TpccAMeminitStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tpcc_b_meminit_status(&self) -> TpccBMeminitStatusR {
        TpccBMeminitStatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
RESERVED: Dont Use"]
    #[inline(always)]
    pub fn tpcc_c_meminit_status(&self) -> TpccCMeminitStatusR {
        TpccCMeminitStatusR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_a_meminit_status(&mut self) -> TpccAMeminitStatusW<DssTpccMeminitStatusSpec> {
        TpccAMeminitStatusW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_b_meminit_status(&mut self) -> TpccBMeminitStatusW<DssTpccMeminitStatusSpec> {
        TpccBMeminitStatusW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
RESERVED: Dont Use"]
    #[inline(always)]
    #[must_use]
    pub fn tpcc_c_meminit_status(&mut self) -> TpccCMeminitStatusW<DssTpccMeminitStatusSpec> {
        TpccCMeminitStatusW::new(self, 2)
    }
}
#[doc = "DSS_TPCC_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_tpcc_meminit_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_tpcc_meminit_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssTpccMeminitStatusSpec;
impl crate::RegisterSpec for DssTpccMeminitStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_tpcc_meminit_status::R`](R) reader structure"]
impl crate::Readable for DssTpccMeminitStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_tpcc_meminit_status::W`](W) writer structure"]
impl crate::Writable for DssTpccMeminitStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_TPCC_MEMINIT_STATUS to value 0"]
impl crate::Resettable for DssTpccMeminitStatusSpec {
    const RESET_VALUE: u32 = 0;
}
