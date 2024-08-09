#[doc = "Register `DSS_L3RAM_MEMINIT_STATUS` reader"]
pub type R = crate::R<DssL3ramMeminitStatusSpec>;
#[doc = "Register `DSS_L3RAM_MEMINIT_STATUS` writer"]
pub type W = crate::W<DssL3ramMeminitStatusSpec>;
#[doc = "Field `l3ram0_meminit_status` reader - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
pub type L3ram0MeminitStatusR = crate::BitReader;
#[doc = "Field `l3ram0_meminit_status` writer - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
pub type L3ram0MeminitStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l3ram1_meminit_status` reader - 1:1\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
pub type L3ram1MeminitStatusR = crate::BitReader;
#[doc = "Field `l3ram1_meminit_status` writer - 1:1\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
pub type L3ram1MeminitStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l3ram2_meminit_status` reader - 2:2\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
pub type L3ram2MeminitStatusR = crate::BitReader;
#[doc = "Field `l3ram2_meminit_status` writer - 2:2\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
pub type L3ram2MeminitStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l3ram3_meminit_status` reader - 3:3\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
pub type L3ram3MeminitStatusR = crate::BitReader;
#[doc = "Field `l3ram3_meminit_status` writer - 3:3\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
pub type L3ram3MeminitStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
    #[inline(always)]
    pub fn l3ram0_meminit_status(&self) -> L3ram0MeminitStatusR {
        L3ram0MeminitStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
    #[inline(always)]
    pub fn l3ram1_meminit_status(&self) -> L3ram1MeminitStatusR {
        L3ram1MeminitStatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
    #[inline(always)]
    pub fn l3ram2_meminit_status(&self) -> L3ram2MeminitStatusR {
        L3ram2MeminitStatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
    #[inline(always)]
    pub fn l3ram3_meminit_status(&self) -> L3ram3MeminitStatusR {
        L3ram3MeminitStatusR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
    #[inline(always)]
    #[must_use]
    pub fn l3ram0_meminit_status(&mut self) -> L3ram0MeminitStatusW<DssL3ramMeminitStatusSpec> {
        L3ram0MeminitStatusW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
    #[inline(always)]
    #[must_use]
    pub fn l3ram1_meminit_status(&mut self) -> L3ram1MeminitStatusW<DssL3ramMeminitStatusSpec> {
        L3ram1MeminitStatusW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
    #[inline(always)]
    #[must_use]
    pub fn l3ram2_meminit_status(&mut self) -> L3ram2MeminitStatusW<DssL3ramMeminitStatusSpec> {
        L3ram2MeminitStatusW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Status field. Read value 0x1 indicates previously triggered Memory intialization of TPCC A Param memory is in progress."]
    #[inline(always)]
    #[must_use]
    pub fn l3ram3_meminit_status(&mut self) -> L3ram3MeminitStatusW<DssL3ramMeminitStatusSpec> {
        L3ram3MeminitStatusW::new(self, 3)
    }
}
#[doc = "DSS_L3RAM_MEMINIT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`dss_l3ram_meminit_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dss_l3ram_meminit_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DssL3ramMeminitStatusSpec;
impl crate::RegisterSpec for DssL3ramMeminitStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dss_l3ram_meminit_status::R`](R) reader structure"]
impl crate::Readable for DssL3ramMeminitStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`dss_l3ram_meminit_status::W`](W) writer structure"]
impl crate::Writable for DssL3ramMeminitStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSS_L3RAM_MEMINIT_STATUS to value 0"]
impl crate::Resettable for DssL3ramMeminitStatusSpec {
    const RESET_VALUE: u32 = 0;
}
