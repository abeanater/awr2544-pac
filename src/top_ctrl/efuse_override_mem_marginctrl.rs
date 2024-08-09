#[doc = "Register `EFUSE_OVERRIDE_MEM_MARGINCTRL` reader"]
pub type R = crate::R<EfuseOverrideMemMarginctrlSpec>;
#[doc = "Register `EFUSE_OVERRIDE_MEM_MARGINCTRL` writer"]
pub type W = crate::W<EfuseOverrideMemMarginctrlSpec>;
#[doc = "Field `glg_margin_override` reader - 2:0\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
pub type GlgMarginOverrideR = crate::FieldReader;
#[doc = "Field `glg_margin_override` writer - 2:0\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
pub type GlgMarginOverrideW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `glg_margin` reader - 5:4\\]
Override MMR value"]
pub type GlgMarginR = crate::FieldReader;
#[doc = "Field `glg_margin` writer - 5:4\\]
Override MMR value"]
pub type GlgMarginW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `gwg_margin_override` reader - 10:8\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
pub type GwgMarginOverrideR = crate::FieldReader;
#[doc = "Field `gwg_margin_override` writer - 10:8\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
pub type GwgMarginOverrideW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `gwg_margin` reader - 15:12\\]
Override MMR value"]
pub type GwgMarginR = crate::FieldReader;
#[doc = "Field `gwg_margin` writer - 15:12\\]
Override MMR value"]
pub type GwgMarginW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `byg_margin_override` reader - 18:16\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
pub type BygMarginOverrideR = crate::FieldReader;
#[doc = "Field `byg_margin_override` writer - 18:16\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
pub type BygMarginOverrideW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `byg_margin` reader - 21:20\\]
Override MMR value"]
pub type BygMarginR = crate::FieldReader;
#[doc = "Field `byg_margin` writer - 21:20\\]
Override MMR value"]
pub type BygMarginW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `brg_margin_override` reader - 26:24\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
pub type BrgMarginOverrideR = crate::FieldReader;
#[doc = "Field `brg_margin_override` writer - 26:24\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
pub type BrgMarginOverrideW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `brg_margin` reader - 29:28\\]
Override MMR value"]
pub type BrgMarginR = crate::FieldReader;
#[doc = "Field `brg_margin` writer - 29:28\\]
Override MMR value"]
pub type BrgMarginW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
    #[inline(always)]
    pub fn glg_margin_override(&self) -> GlgMarginOverrideR {
        GlgMarginOverrideR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Override MMR value"]
    #[inline(always)]
    pub fn glg_margin(&self) -> GlgMarginR {
        GlgMarginR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
    #[inline(always)]
    pub fn gwg_margin_override(&self) -> GwgMarginOverrideR {
        GwgMarginOverrideR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Override MMR value"]
    #[inline(always)]
    pub fn gwg_margin(&self) -> GwgMarginR {
        GwgMarginR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
    #[inline(always)]
    pub fn byg_margin_override(&self) -> BygMarginOverrideR {
        BygMarginOverrideR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Override MMR value"]
    #[inline(always)]
    pub fn byg_margin(&self) -> BygMarginR {
        BygMarginR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
    #[inline(always)]
    pub fn brg_margin_override(&self) -> BrgMarginOverrideR {
        BrgMarginOverrideR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Override MMR value"]
    #[inline(always)]
    pub fn brg_margin(&self) -> BrgMarginR {
        BrgMarginR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
    #[inline(always)]
    #[must_use]
    pub fn glg_margin_override(&mut self) -> GlgMarginOverrideW<EfuseOverrideMemMarginctrlSpec> {
        GlgMarginOverrideW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Override MMR value"]
    #[inline(always)]
    #[must_use]
    pub fn glg_margin(&mut self) -> GlgMarginW<EfuseOverrideMemMarginctrlSpec> {
        GlgMarginW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
    #[inline(always)]
    #[must_use]
    pub fn gwg_margin_override(&mut self) -> GwgMarginOverrideW<EfuseOverrideMemMarginctrlSpec> {
        GwgMarginOverrideW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Override MMR value"]
    #[inline(always)]
    #[must_use]
    pub fn gwg_margin(&mut self) -> GwgMarginW<EfuseOverrideMemMarginctrlSpec> {
        GwgMarginW::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
    #[inline(always)]
    #[must_use]
    pub fn byg_margin_override(&mut self) -> BygMarginOverrideW<EfuseOverrideMemMarginctrlSpec> {
        BygMarginOverrideW::new(self, 16)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Override MMR value"]
    #[inline(always)]
    #[must_use]
    pub fn byg_margin(&mut self) -> BygMarginW<EfuseOverrideMemMarginctrlSpec> {
        BygMarginW::new(self, 20)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Override EFUSE Value with SW Value Write 3'b000 : EFUSE Value Write 3'b111 : MMR Value"]
    #[inline(always)]
    #[must_use]
    pub fn brg_margin_override(&mut self) -> BrgMarginOverrideW<EfuseOverrideMemMarginctrlSpec> {
        BrgMarginOverrideW::new(self, 24)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Override MMR value"]
    #[inline(always)]
    #[must_use]
    pub fn brg_margin(&mut self) -> BrgMarginW<EfuseOverrideMemMarginctrlSpec> {
        BrgMarginW::new(self, 28)
    }
}
#[doc = "EFUSE_OVERRIDE_MEM_MARGINCTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_override_mem_marginctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_override_mem_marginctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfuseOverrideMemMarginctrlSpec;
impl crate::RegisterSpec for EfuseOverrideMemMarginctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse_override_mem_marginctrl::R`](R) reader structure"]
impl crate::Readable for EfuseOverrideMemMarginctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`efuse_override_mem_marginctrl::W`](W) writer structure"]
impl crate::Writable for EfuseOverrideMemMarginctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSE_OVERRIDE_MEM_MARGINCTRL to value 0"]
impl crate::Resettable for EfuseOverrideMemMarginctrlSpec {
    const RESET_VALUE: u32 = 0;
}
