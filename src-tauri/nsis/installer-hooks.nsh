; -----------------------------------------------------------------------------
; AIRust_MT custom NSIS installer hooks
;
;  1. Language page: selects the APP UI language (shown before the welcome
;     page). The installer UI itself always follows the Windows system
;     language (NSIS maps it automatically; displayLanguageSelector is false
;     in tauri.conf.json).
;
;     The page clearly reminds the user that the choice applies to the
;     software after installation, NOT to the installer. The chosen code is
;     written to HKCU\Software\AIRust_MT\InstallLang in
;     NSIS_HOOK_POSTINSTALL and picked up by the app on first launch
;     (src-tauri/src/settings.rs get_install_lang).
;
;     We deliberately do NOT touch $LANGUAGE: NSIS finalizes the installer
;     language right after .onInit, so it cannot be switched from a page.
;
;  2. Optional page with a checkbox "Associate Markdown (.md) files with
;     AIRust_MT" (checked by default). The associations are only written to
;     the registry when the box is checked, and are cleaned up on uninstall.
;
; NOTE: this file is saved as UTF-8 WITH BOM so makensis reads the
;       non-ASCII LangString texts correctly.
; -----------------------------------------------------------------------------

; App language codes must match src/i18n.ts (UI_LANGS)

LangString AirustAppLangTitle 1033 "Software language"
LangString AirustAppLangTitle 2052 "软件语言"
LangString AirustAppLangTitle 1041 "ソフトウェアの言語"
LangString AirustAppLangTitle 1042 "소프트웨어 언어"
LangString AirustAppLangTitle 1049 "Язык программы"
LangString AirustAppLangTitle 1034 "Idioma del software"
LangString AirustAppLangTitle 1036 "Langue du logiciel"

LangString AirustAppLangReminder 1033 "This only sets the interface language of the software after installation. The installer itself uses your system language."
LangString AirustAppLangReminder 2052 "此处选择的只是软件安装完成后的界面语言，安装过程本身的语言将跟随系统语言。"
LangString AirustAppLangReminder 1041 "ここで選択するのは、インストール後に使うソフトウェアの表示言語のみです。インストーラー自体はシステム言語のままです。"
LangString AirustAppLangReminder 1042 "여기에서 선택하는 것은 설치 후 소프트웨어의 인터페이스 언어뿐입니다. 설치 프로그램 자체는 시스템 언어를 따릅니다."
LangString AirustAppLangReminder 1049 "Здесь выбирается только язык интерфейса программы после установки; сам установщик будет использовать системный язык."
LangString AirustAppLangReminder 1034 "Aquí solo se elige el idioma de la interfaz del programa después de la instalación; el instalador seguirá usando el idioma del sistema."
LangString AirustAppLangReminder 1036 "Ce choix ne définit que la langue de l'interface du logiciel après l'installation ; l'installateur lui-même utilise la langue du système."

; ---------------------------------------------------------------------------
; Map an installer language id to the app language code.
; Input: $R8 = language id (e.g. "2052"), Output: $R9 = app code (e.g. "zh-CN")
; ---------------------------------------------------------------------------
Function MapLangIdToAppCode
  StrCpy $R9 "en"
  ${If} $R8 == "2052"
    StrCpy $R9 "zh-CN"
  ${ElseIf} $R8 == "1041"
    StrCpy $R9 "ja"
  ${ElseIf} $R8 == "1042"
    StrCpy $R9 "ko"
  ${ElseIf} $R8 == "1049"
    StrCpy $R9 "ru"
  ${ElseIf} $R8 == "1034"
    StrCpy $R9 "es"
  ${ElseIf} $R8 == "1036"
    StrCpy $R9 "fr"
  ${EndIf}
FunctionEnd

; ---------------------------------------------------------------------------
; Page 1: app language selection (default = current system language)
; ---------------------------------------------------------------------------
Var LanguageList
Var AppInstallLang

Page custom langSelCreate langSelLeave

Function langSelCreate
  ; Passive mode (/P): no interaction, keep the system language mapping.
  ; Note: the hook file is included before the template declares
  ; $PassiveMode, so we re-parse the command line instead.
  ${GetOptions} $CMDLINE "/P" $0
  ${IfNot} ${Errors}
    StrCpy $R8 $LANGUAGE
    Call MapLangIdToAppCode
    StrCpy $AppInstallLang $R9
    Abort
  ${EndIf}

  !insertmacro MUI_HEADER_TEXT "$(AirustAppLangTitle)" ""

  nsDialogs::Create 1018
  Pop $0
  ${If} $0 == error
    Abort
  ${EndIf}

  ${NSD_CreateLabel} 0 10u 100% 26u "$(AirustAppLangReminder)"
  Pop $0

  ${NSD_CreateDropList} 0 44u -30u 30u ""
  Pop $LanguageList
  ${NSD_CB_AddString} $LanguageList "English"
  ${NSD_CB_AddString} $LanguageList "Chinese (Simplified)"
  ${NSD_CB_AddString} $LanguageList "Japanese"
  ${NSD_CB_AddString} $LanguageList "Korean"
  ${NSD_CB_AddString} $LanguageList "Russian"
  ${NSD_CB_AddString} $LanguageList "Spanish"
  ${NSD_CB_AddString} $LanguageList "French"

  ; Preselect the language matching the system language
  StrCpy $R8 $LANGUAGE
  Call MapLangIdToAppCode
  StrCpy $1 "English"
  ${If} $R9 == "zh-CN"
    StrCpy $1 "Chinese (Simplified)"
  ${ElseIf} $R9 == "ja"
    StrCpy $1 "Japanese"
  ${ElseIf} $R9 == "ko"
    StrCpy $1 "Korean"
  ${ElseIf} $R9 == "ru"
    StrCpy $1 "Russian"
  ${ElseIf} $R9 == "es"
    StrCpy $1 "Spanish"
  ${ElseIf} $R9 == "fr"
    StrCpy $1 "French"
  ${EndIf}
  ${NSD_CB_SelectString} $LanguageList $1

  nsDialogs::Show
FunctionEnd

Function langSelLeave
  ${NSD_GetText} $LanguageList $0
  StrCpy $AppInstallLang "en"
  ${If} $0 == "Chinese (Simplified)"
    StrCpy $AppInstallLang "zh-CN"
  ${ElseIf} $0 == "Japanese"
    StrCpy $AppInstallLang "ja"
  ${ElseIf} $0 == "Korean"
    StrCpy $AppInstallLang "ko"
  ${ElseIf} $0 == "Russian"
    StrCpy $AppInstallLang "ru"
  ${ElseIf} $0 == "Spanish"
    StrCpy $AppInstallLang "es"
  ${ElseIf} $0 == "French"
    StrCpy $AppInstallLang "fr"
  ${EndIf}
FunctionEnd

; ---------------------------------------------------------------------------
; Page 2: optional .md file association (default checked)
; ---------------------------------------------------------------------------
Var MdCheckbox
Var MdAssocState

Page custom assocCreate assocLeave

Function assocCreate
  !insertmacro MUI_HEADER_TEXT "Options" "Additional install options"

  nsDialogs::Create 1018
  Pop $0
  ${If} $0 == error
    Abort
  ${EndIf}

  ${NSD_CreateCheckBox} 0 25u 100% 12u "Associate Markdown (.md) files with AIRust_MT"
  Pop $MdCheckbox
  ${NSD_Check} $MdCheckbox

  nsDialogs::Show
FunctionEnd

Function assocLeave
  ${NSD_GetState} $MdCheckbox $1
  StrCpy $MdAssocState $1
FunctionEnd

; ---------------------------------------------------------------------------
; Install: save the selected app language and register associations when
; checked. Also save the installer (system) language so the uninstaller
; shows in the same language without prompting.
; ---------------------------------------------------------------------------
!macro NSIS_HOOK_POSTINSTALL
  ; Fallback for silent installs where the language page never ran
  ${If} $AppInstallLang == ""
    StrCpy $R8 $LANGUAGE
    Call MapLangIdToAppCode
    StrCpy $AppInstallLang $R9
  ${EndIf}
  WriteRegStr HKCU "Software\AIRust_MT" "InstallLang" "$AppInstallLang"
  ; Uninstaller language (read back by MUI_UNGETLANGUAGE in un.onInit)
  WriteRegStr HKCU "${MANUPRODUCTKEY}" "Installer Language" "$LANGUAGE"

  ${If} $MdAssocState = 1
    !insertmacro APP_ASSOCIATE "md" "AIRust_MT.md" "Markdown Document" "$INSTDIR\airust_mt.exe,0" "Open with AIRust_MT" "$\"$INSTDIR\airust_mt.exe$\" $\"%1$\""
    !insertmacro APP_ASSOCIATE "markdown" "AIRust_MT.md" "Markdown Document" "$INSTDIR\airust_mt.exe,0" "Open with AIRust_MT" "$\"$INSTDIR\airust_mt.exe$\" $\"%1$\""
    !insertmacro APP_ASSOCIATE "mdown" "AIRust_MT.md" "Markdown Document" "$INSTDIR\airust_mt.exe,0" "Open with AIRust_MT" "$\"$INSTDIR\airust_mt.exe$\" $\"%1$\""
    !insertmacro APP_ASSOCIATE "mkd" "AIRust_MT.md" "Markdown Document" "$INSTDIR\airust_mt.exe,0" "Open with AIRust_MT" "$\"$INSTDIR\airust_mt.exe$\" $\"%1$\""
    !insertmacro APP_ASSOCIATE "mkdn" "AIRust_MT.md" "Markdown Document" "$INSTDIR\airust_mt.exe,0" "Open with AIRust_MT" "$\"$INSTDIR\airust_mt.exe$\" $\"%1$\""
    !insertmacro UPDATEFILEASSOC
  ${EndIf}
!macroend

; ---------------------------------------------------------------------------
; Uninstall: remove associations only if they belong to AIRust_MT
; ---------------------------------------------------------------------------
!macro NSIS_HOOK_PREUNINSTALL
  ; Clean up the "install language" registry key written during install
  DeleteRegKey HKCU "Software\AIRust_MT"
  ReadRegStr $0 SHELL_CONTEXT "Software\Classes\.md" ""
  ${If} $0 == "AIRust_MT.md"
    !insertmacro APP_UNASSOCIATE "md" "AIRust_MT.md"
  ${EndIf}
  ReadRegStr $0 SHELL_CONTEXT "Software\Classes\.markdown" ""
  ${If} $0 == "AIRust_MT.md"
    !insertmacro APP_UNASSOCIATE "markdown" "AIRust_MT.md"
  ${EndIf}
  ReadRegStr $0 SHELL_CONTEXT "Software\Classes\.mdown" ""
  ${If} $0 == "AIRust_MT.md"
    !insertmacro APP_UNASSOCIATE "mdown" "AIRust_MT.md"
  ${EndIf}
  ReadRegStr $0 SHELL_CONTEXT "Software\Classes\.mkd" ""
  ${If} $0 == "AIRust_MT.md"
    !insertmacro APP_UNASSOCIATE "mkd" "AIRust_MT.md"
  ${EndIf}
  ReadRegStr $0 SHELL_CONTEXT "Software\Classes\.mkdn" ""
  ${If} $0 == "AIRust_MT.md"
    !insertmacro APP_UNASSOCIATE "mkdn" "AIRust_MT.md"
  ${EndIf}
!macroend
