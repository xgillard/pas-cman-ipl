Outfile "PasCmanInstaller.exe"

InstallDir "$PROGRAMFILES\PasCman"

Page directory
Page instfiles

Section "PasCman (required)"
  SetOutPath $INSTDIR

  ; Ajoute les fichiers de ton jeu
  ; Assure-toi que les chemins des fichiers sont corrects
  File "target\release\pas-cman.exe"

  CreateDirectory "$INSTDIR\resources"
  SetOutPath $INSTDIR\resources
  File "resources\map.txt"
  File "resources\pas-cman-font-32.png"
  File "resources\terminal8x8.png"

  SetOutPath $INSTDIR
  CreateDirectory "$SMPROGRAMS\PasCman"
  CreateShortCut "$SMPROGRAMS\PasCman\PasCman.lnk" "$INSTDIR\pas-cman.exe"

  CreateShortCut "$DESKTOP\PasCman.lnk" "$INSTDIR\pas-cman.exe"

  WriteUninstaller "$INSTDIR\uninstall.exe"
  CreateShortCut "$SMPROGRAMS\PasCman\uninstall.lnk" "$INSTDIR\uninstall.exe"
SectionEnd

Section "Uninstall"

  Delete "$INSTDIR\pas-cman.exe"
  Delete "$INSTDIR\resources\map.txt"
  Delete "$INSTDIR\resources\pas-cman-font-32.png"
  Delete "$INSTDIR\resources\terminal8x8.png"

  RMDir /r "$INSTDIR"

  ; Supprime les raccourcis et le répertoire dans le menu Démarrer
  Delete "$SMPROGRAMS\PasCman\PasCman.lnk"
  RMDir "$SMPROGRAMS\PasCman"

  ; Supprime les raccourcis sur le bureau
  Delete "$DESKTOP\PasCman.lnk"

SectionEnd