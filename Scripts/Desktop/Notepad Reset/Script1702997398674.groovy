import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

Windows.startApplicationWithTitle('C:\\Windows\\System32\\notepad.exe', 'Notepad')

Windows.setText(findWindowsObject('Object Repository/Notepad/Edit'), 'Resetting Notepad Font...')

Windows.click(findWindowsObject('Object Repository/Notepad/MenuItem'))

Windows.click(findWindowsObject('Object Repository/Notepad/MenuItem(1)'))

Windows.setText(findWindowsObject('Object Repository/Notepad/Edit(1)'), 'Calibri')

Windows.click(findWindowsObject('Object Repository/Notepad/ListItem'))

Windows.setText(findWindowsObject('Object Repository/Notepad/Edit(2)'), '12')

Windows.click(findWindowsObject('Object Repository/Notepad/Button'))

Windows.setText(findWindowsObject('Object Repository/Notepad/Edit'), '\r\nDone :D')

Windows.click(findWindowsObject('Object Repository/Notepad/MenuItem(2)'))

Windows.click(findWindowsObject('Object Repository/Notepad/MenuItem(3)'))

Windows.click(findWindowsObject('Object Repository/Notepad/Button(1)'))

