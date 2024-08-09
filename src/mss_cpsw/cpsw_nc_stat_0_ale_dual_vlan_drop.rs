#[doc = "Register `CPSW_NC_STAT_0_ALE_DUAL_VLAN_DROP` reader"]
pub type R = crate::R<CpswNcStat0AleDualVlanDropSpec>;
#[doc = "Register `CPSW_NC_STAT_0_ALE_DUAL_VLAN_DROP` writer"]
pub type W = crate::W<CpswNcStat0AleDualVlanDropSpec>;
#[doc = "Field `ALE_DUAL_VLAN` reader - 31:0\\]
ALE Dual VLAN drop"]
pub type AleDualVlanR = crate::FieldReader<u32>;
#[doc = "Field `ALE_DUAL_VLAN` writer - 31:0\\]
ALE Dual VLAN drop"]
pub type AleDualVlanW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
ALE Dual VLAN drop"]
    #[inline(always)]
    pub fn ale_dual_vlan(&self) -> AleDualVlanR {
        AleDualVlanR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
ALE Dual VLAN drop"]
    #[inline(always)]
    #[must_use]
    pub fn ale_dual_vlan(&mut self) -> AleDualVlanW<CpswNcStat0AleDualVlanDropSpec> {
        AleDualVlanW::new(self, 0)
    }
}
#[doc = "ALE Dual VLAN Drop\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_stat_0_ale_dual_vlan_drop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_stat_0_ale_dual_vlan_drop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcStat0AleDualVlanDropSpec;
impl crate::RegisterSpec for CpswNcStat0AleDualVlanDropSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_stat_0_ale_dual_vlan_drop::R`](R) reader structure"]
impl crate::Readable for CpswNcStat0AleDualVlanDropSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_stat_0_ale_dual_vlan_drop::W`](W) writer structure"]
impl crate::Writable for CpswNcStat0AleDualVlanDropSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_STAT_0_ALE_DUAL_VLAN_DROP to value 0"]
impl crate::Resettable for CpswNcStat0AleDualVlanDropSpec {
    const RESET_VALUE: u32 = 0;
}
