#[doc = "Register `MSTIDWRENA` reader"]
pub type R = crate::R<MstidwrenaSpec>;
#[doc = "Register `MSTIDWRENA` writer"]
pub type W = crate::W<MstidwrenaSpec>;
#[doc = "Field `MSTIDREG_WRENA` reader - 3:0\\]
Readable in both user and privileged modes. 1010 = All master-id registers are unlocked and available for write. others = Writes to all master-id registers are locked. Writable only in privileged mode 1010 = Writes to master-id registers are unlocked. others = Writes to master-id registers are locked."]
pub type MstidregWrenaR = crate::FieldReader;
#[doc = "Field `MSTIDREG_WRENA` writer - 3:0\\]
Readable in both user and privileged modes. 1010 = All master-id registers are unlocked and available for write. others = Writes to all master-id registers are locked. Writable only in privileged mode 1010 = Writes to master-id registers are unlocked. others = Writes to master-id registers are locked."]
pub type MstidregWrenaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Readable in both user and privileged modes. 1010 = All master-id registers are unlocked and available for write. others = Writes to all master-id registers are locked. Writable only in privileged mode 1010 = Writes to master-id registers are unlocked. others = Writes to master-id registers are locked."]
    #[inline(always)]
    pub fn mstidreg_wrena(&self) -> MstidregWrenaR {
        MstidregWrenaR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Readable in both user and privileged modes. 1010 = All master-id registers are unlocked and available for write. others = Writes to all master-id registers are locked. Writable only in privileged mode 1010 = Writes to master-id registers are unlocked. others = Writes to master-id registers are locked."]
    #[inline(always)]
    #[must_use]
    pub fn mstidreg_wrena(&mut self) -> MstidregWrenaW<MstidwrenaSpec> {
        MstidregWrenaW::new(self, 0)
    }
}
#[doc = "MasterID Protection Write Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mstidwrena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstidwrena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstidwrenaSpec;
impl crate::RegisterSpec for MstidwrenaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstidwrena::R`](R) reader structure"]
impl crate::Readable for MstidwrenaSpec {}
#[doc = "`write(|w| ..)` method takes [`mstidwrena::W`](W) writer structure"]
impl crate::Writable for MstidwrenaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSTIDWRENA to value 0"]
impl crate::Resettable for MstidwrenaSpec {
    const RESET_VALUE: u32 = 0;
}
