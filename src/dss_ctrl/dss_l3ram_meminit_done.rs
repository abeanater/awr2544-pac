#[doc = "Register `DSS_L3RAM_MEMINIT_DONE` reader"]
pub type R = crate::R<DssL3ramMeminitDoneSpec>;
#[doc = "Register `DSS_L3RAM_MEMINIT_DONE` writer"]
pub type W = crate::W<DssL3ramMeminitDoneSpec>;
#[doc = "Field `l3ram0_meminit_done` reader - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
pub type L3ram0MeminitDoneR = crate::BitReader;
#[doc = "Field `l3ram0_meminit_done` writer - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
pub type L3ram0MeminitDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l3ram1_meminit_done` reader - 1:1\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
pub type L3ram1MeminitDoneR = crate::BitReader;
#[doc = "Field `l3ram1_meminit_done` writer - 1:1\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
pub type L3ram1MeminitDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l3ram2_meminit_done` reader - 2:2\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
pub type L3ram2MeminitDoneR = crate::BitReader;
#[doc = "Field `l3ram2_meminit_done` writer - 2:2\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
pub type L3ram2MeminitDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l3ram3_meminit_done` reader - 3:3\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
pub type L3ram3MeminitDoneR = crate::BitReader;
#[doc = "Field `l3ram3_meminit_done` writer - 3:3\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
pub type L3ram3MeminitDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
    #[inline(always)]
    pub fn l3ram0_meminit_done(&self) -> L3ram0MeminitDoneR {
        L3ram0MeminitDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
    #[inline(always)]
    pub fn l3ram1_meminit_done(&self) -> L3ram1MeminitDoneR {
        L3ram1MeminitDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
    #[inline(always)]
    pub fn l3ram2_meminit_done(&self) -> L3ram2MeminitDoneR {
        L3ram2MeminitDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
    #[inline(always)]
    pub fn l3ram3_meminit_done(&self) -> L3ram3MeminitDoneR {
        L3ram3MeminitDoneR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
    #[inline(always)]
    #[must_use]
    pub fn l3ram0_meminit_done(&mut self) -> L3ram0MeminitDoneW<DssL3ramMeminitDoneSpec> {
        L3ram0MeminitDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
    #[inline(always)]
    #[must_use]
    pub fn l3ram1_meminit_done(&mut self) -> L3ram1MeminitDoneW<DssL3ramMeminitDoneSpec> {
        L3ram1MeminitDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
    #[inline(always)]
    #[must_use]
    pub fn l3ram2_meminit_done(&mut self) -> L3ram2MeminitDoneW<DssL3ramMeminitDoneSpec> {
        L3ram2MeminitDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is complte. Write 0x1 to clear status. Refer TPCC Memory initialization sequnce in EDMA section for more details"]
    #[inline(always)]
    #[must_use]
    pub fn l3ram3_meminit_done(&mut self) -> L3ram3MeminitDoneW<DssL3ramMeminitDoneSpec> {
        L3ram3MeminitDoneW::new(self, 3)
    }
}
#[doc = "DSS_L3RAM_MEMINIT_DONE\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_l3ram_meminit_done::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_l3ram_meminit_done::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssL3ramMeminitDoneSpec;
impl crate::RegisterSpec for DssL3ramMeminitDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_l3ram_meminit_done::R`](R) reader structure"]
impl crate::Readable for DssL3ramMeminitDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_l3ram_meminit_done::W`](W) writer structure"]
impl crate::Writable for DssL3ramMeminitDoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_L3RAM_MEMINIT_DONE to value 0"]
impl crate::Resettable for DssL3ramMeminitDoneSpec {
    const RESET_VALUE: u32 = 0;
}
