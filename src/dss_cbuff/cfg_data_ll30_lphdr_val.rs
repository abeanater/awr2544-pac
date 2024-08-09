#[doc = "Register `CFG_DATA_LL30_LPHDR_VAL` reader"]
pub type R = crate::R<CfgDataLl30LphdrValSpec>;
#[doc = "Register `CFG_DATA_LL30_LPHDR_VAL` writer"]
pub type W = crate::W<CfgDataLl30LphdrValSpec>;
#[doc = "Field `LL30_LPHDR_VAL` reader - 31:0\\]
CSI-2 Programming : Configure the Long Packet Header to be sent to the Protocol Engine if the LPHDR_EN field is set for the linklist. LVDS Programming : Configure with the static value : 0xBBBBBBBB"]
pub type Ll30LphdrValR = crate::FieldReader<u32>;
#[doc = "Field `LL30_LPHDR_VAL` writer - 31:0\\]
CSI-2 Programming : Configure the Long Packet Header to be sent to the Protocol Engine if the LPHDR_EN field is set for the linklist. LVDS Programming : Configure with the static value : 0xBBBBBBBB"]
pub type Ll30LphdrValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
CSI-2 Programming : Configure the Long Packet Header to be sent to the Protocol Engine if the LPHDR_EN field is set for the linklist. LVDS Programming : Configure with the static value : 0xBBBBBBBB"]
    #[inline(always)]
    pub fn ll30_lphdr_val(&self) -> Ll30LphdrValR {
        Ll30LphdrValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
CSI-2 Programming : Configure the Long Packet Header to be sent to the Protocol Engine if the LPHDR_EN field is set for the linklist. LVDS Programming : Configure with the static value : 0xBBBBBBBB"]
    #[inline(always)]
    #[must_use]
    pub fn ll30_lphdr_val(&mut self) -> Ll30LphdrValW<CfgDataLl30LphdrValSpec> {
        Ll30LphdrValW::new(self, 0)
    }
}
#[doc = "CFG_DATA_LL30_LPHDR_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_data_ll30_lphdr_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_data_ll30_lphdr_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDataLl30LphdrValSpec;
impl crate::RegisterSpec for CfgDataLl30LphdrValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_data_ll30_lphdr_val::R`](R) reader structure"]
impl crate::Readable for CfgDataLl30LphdrValSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_data_ll30_lphdr_val::W`](W) writer structure"]
impl crate::Writable for CfgDataLl30LphdrValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DATA_LL30_LPHDR_VAL to value 0"]
impl crate::Resettable for CfgDataLl30LphdrValSpec {
    const RESET_VALUE: u32 = 0;
}
