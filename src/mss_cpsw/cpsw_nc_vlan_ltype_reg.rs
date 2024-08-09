#[doc = "Register `CPSW_NC_VLAN_LTYPE_REG` reader"]
pub type R = crate::R<CpswNcVlanLtypeRegSpec>;
#[doc = "Register `CPSW_NC_VLAN_LTYPE_REG` writer"]
pub type W = crate::W<CpswNcVlanLtypeRegSpec>;
#[doc = "Field `INNER_VLAN_LTYPE` reader - 15:0\\]
Inner VLAN LType"]
pub type InnerVlanLtypeR = crate::FieldReader<u16>;
#[doc = "Field `INNER_VLAN_LTYPE` writer - 15:0\\]
Inner VLAN LType"]
pub type InnerVlanLtypeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OUTER_VLAN_LTYPE` reader - 31:16\\]
Outer VLAN LType"]
pub type OuterVlanLtypeR = crate::FieldReader<u16>;
#[doc = "Field `OUTER_VLAN_LTYPE` writer - 31:16\\]
Outer VLAN LType"]
pub type OuterVlanLtypeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Inner VLAN LType"]
    #[inline(always)]
    pub fn inner_vlan_ltype(&self) -> InnerVlanLtypeR {
        InnerVlanLtypeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Outer VLAN LType"]
    #[inline(always)]
    pub fn outer_vlan_ltype(&self) -> OuterVlanLtypeR {
        OuterVlanLtypeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Inner VLAN LType"]
    #[inline(always)]
    #[must_use]
    pub fn inner_vlan_ltype(&mut self) -> InnerVlanLtypeW<CpswNcVlanLtypeRegSpec> {
        InnerVlanLtypeW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Outer VLAN LType"]
    #[inline(always)]
    #[must_use]
    pub fn outer_vlan_ltype(&mut self) -> OuterVlanLtypeW<CpswNcVlanLtypeRegSpec> {
        OuterVlanLtypeW::new(self, 16)
    }
}
#[doc = "VLAN Length/type\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsw_nc_vlan_ltype_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsw_nc_vlan_ltype_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNcVlanLtypeRegSpec;
impl crate::RegisterSpec for CpswNcVlanLtypeRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nc_vlan_ltype_reg::R`](R) reader structure"]
impl crate::Readable for CpswNcVlanLtypeRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nc_vlan_ltype_reg::W`](W) writer structure"]
impl crate::Writable for CpswNcVlanLtypeRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NC_VLAN_LTYPE_REG to value 0"]
impl crate::Resettable for CpswNcVlanLtypeRegSpec {
    const RESET_VALUE: u32 = 0;
}
