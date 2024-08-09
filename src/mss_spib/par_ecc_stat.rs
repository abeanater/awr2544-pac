#[doc = "Register `PAR_ECC_STAT` reader"]
pub type R = crate::R<ParEccStatSpec>;
#[doc = "Register `PAR_ECC_STAT` writer"]
pub type W = crate::W<ParEccStatSpec>;
#[doc = "Field `UERR_FLG0` reader - 0:0\\]
Uncorrectable Parity or double bit ECC error detection flag This flag indicates if a Parity or ECC error ocurred on reading TXRAM When this bit is read: 0 = No error occured. 1 = Error detected and the address is captured in UERRADDR0 register. When write to this bit with: 0 = No effect. 1 = Clears the bit."]
pub type UerrFlg0R = crate::BitReader;
#[doc = "Field `UERR_FLG0` writer - 0:0\\]
Uncorrectable Parity or double bit ECC error detection flag This flag indicates if a Parity or ECC error ocurred on reading TXRAM When this bit is read: 0 = No error occured. 1 = Error detected and the address is captured in UERRADDR0 register. When write to this bit with: 0 = No effect. 1 = Clears the bit."]
pub type UerrFlg0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UERR_FLG1` reader - 1:1\\]
Uncorrectable Parity or double bit ECC error detection flag This flag indicates if a Parity or double bit ECC error ocurred on reading RXRAM When this bit is read: 0 = No error occured. 1 = Error detected and the address is captured in UERRADDR1 register. When write to this bit with: 0 = No effect. 1 = Clears the bit"]
pub type UerrFlg1R = crate::BitReader;
#[doc = "Field `UERR_FLG1` writer - 1:1\\]
Uncorrectable Parity or double bit ECC error detection flag This flag indicates if a Parity or double bit ECC error ocurred on reading RXRAM When this bit is read: 0 = No error occured. 1 = Error detected and the address is captured in UERRADDR1 register. When write to this bit with: 0 = No effect. 1 = Clears the bit"]
pub type UerrFlg1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - 7:2\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - 7:2\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SBE_FLG0` reader - 8:8\\]
Single Bit Error in TXRAM. This flag indicates if a single bit ECC Error ocurred on reading TXRAM Read: 0 = No error occured. 1 = Single bit error is detected in TXRAM and the address is captured in SBERRADDR0 register. Write: 0 = No effect. 1 = Clears the bit."]
pub type SbeFlg0R = crate::BitReader;
#[doc = "Field `SBE_FLG0` writer - 8:8\\]
Single Bit Error in TXRAM. This flag indicates if a single bit ECC Error ocurred on reading TXRAM Read: 0 = No error occured. 1 = Single bit error is detected in TXRAM and the address is captured in SBERRADDR0 register. Write: 0 = No effect. 1 = Clears the bit."]
pub type SbeFlg0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBE_FLG1` reader - 9:9\\]
Single Bit Error in RXRAM. This flag indicates if a single bit ECC Error ocurred on reading RXRAM Read: 0 = No error occured. 1 = Single bit error is detected in RXRAM and the address is captured in SBERRADDR1 register. Write: 0 = No effect. 1 = Clears the bit."]
pub type SbeFlg1R = crate::BitReader;
#[doc = "Field `SBE_FLG1` writer - 9:9\\]
Single Bit Error in RXRAM. This flag indicates if a single bit ECC Error ocurred on reading RXRAM Read: 0 = No error occured. 1 = Single bit error is detected in RXRAM and the address is captured in SBERRADDR1 register. Write: 0 = No effect. 1 = Clears the bit."]
pub type SbeFlg1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - 31:10\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2R = crate::FieldReader<u32>;
#[doc = "Field `NU2` writer - 31:10\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Uncorrectable Parity or double bit ECC error detection flag This flag indicates if a Parity or ECC error ocurred on reading TXRAM When this bit is read: 0 = No error occured. 1 = Error detected and the address is captured in UERRADDR0 register. When write to this bit with: 0 = No effect. 1 = Clears the bit."]
    #[inline(always)]
    pub fn uerr_flg0(&self) -> UerrFlg0R {
        UerrFlg0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Uncorrectable Parity or double bit ECC error detection flag This flag indicates if a Parity or double bit ECC error ocurred on reading RXRAM When this bit is read: 0 = No error occured. 1 = Error detected and the address is captured in UERRADDR1 register. When write to this bit with: 0 = No effect. 1 = Clears the bit"]
    #[inline(always)]
    pub fn uerr_flg1(&self) -> UerrFlg1R {
        UerrFlg1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Single Bit Error in TXRAM. This flag indicates if a single bit ECC Error ocurred on reading TXRAM Read: 0 = No error occured. 1 = Single bit error is detected in TXRAM and the address is captured in SBERRADDR0 register. Write: 0 = No effect. 1 = Clears the bit."]
    #[inline(always)]
    pub fn sbe_flg0(&self) -> SbeFlg0R {
        SbeFlg0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Single Bit Error in RXRAM. This flag indicates if a single bit ECC Error ocurred on reading RXRAM Read: 0 = No error occured. 1 = Single bit error is detected in RXRAM and the address is captured in SBERRADDR1 register. Write: 0 = No effect. 1 = Clears the bit."]
    #[inline(always)]
    pub fn sbe_flg1(&self) -> SbeFlg1R {
        SbeFlg1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Uncorrectable Parity or double bit ECC error detection flag This flag indicates if a Parity or ECC error ocurred on reading TXRAM When this bit is read: 0 = No error occured. 1 = Error detected and the address is captured in UERRADDR0 register. When write to this bit with: 0 = No effect. 1 = Clears the bit."]
    #[inline(always)]
    #[must_use]
    pub fn uerr_flg0(&mut self) -> UerrFlg0W<ParEccStatSpec> {
        UerrFlg0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Uncorrectable Parity or double bit ECC error detection flag This flag indicates if a Parity or double bit ECC error ocurred on reading RXRAM When this bit is read: 0 = No error occured. 1 = Error detected and the address is captured in UERRADDR1 register. When write to this bit with: 0 = No effect. 1 = Clears the bit"]
    #[inline(always)]
    #[must_use]
    pub fn uerr_flg1(&mut self) -> UerrFlg1W<ParEccStatSpec> {
        UerrFlg1W::new(self, 1)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<ParEccStatSpec> {
        Nu1W::new(self, 2)
    }
    #[doc = "Bit 8 - 8:8\\]
Single Bit Error in TXRAM. This flag indicates if a single bit ECC Error ocurred on reading TXRAM Read: 0 = No error occured. 1 = Single bit error is detected in TXRAM and the address is captured in SBERRADDR0 register. Write: 0 = No effect. 1 = Clears the bit."]
    #[inline(always)]
    #[must_use]
    pub fn sbe_flg0(&mut self) -> SbeFlg0W<ParEccStatSpec> {
        SbeFlg0W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Single Bit Error in RXRAM. This flag indicates if a single bit ECC Error ocurred on reading RXRAM Read: 0 = No error occured. 1 = Single bit error is detected in RXRAM and the address is captured in SBERRADDR1 register. Write: 0 = No effect. 1 = Clears the bit."]
    #[inline(always)]
    #[must_use]
    pub fn sbe_flg1(&mut self) -> SbeFlg1W<ParEccStatSpec> {
        SbeFlg1W::new(self, 9)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Reserved.Reads return ΓÇÿ0ΓÇÖ and writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<ParEccStatSpec> {
        Nu2W::new(self, 10)
    }
}
#[doc = "Parity/ECC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`par_ecc_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`par_ecc_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ParEccStatSpec;
impl crate::RegisterSpec for ParEccStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`par_ecc_stat::R`](R) reader structure"]
impl crate::Readable for ParEccStatSpec {}
#[doc = "`write(|w| ..)` method takes [`par_ecc_stat::W`](W) writer structure"]
impl crate::Writable for ParEccStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAR_ECC_STAT to value 0"]
impl crate::Resettable for ParEccStatSpec {
    const RESET_VALUE: u32 = 0;
}
