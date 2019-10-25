<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PersonSearch_EN_Nombre2</name>
   <tag></tag>
   <elementGuidId>ba65305c-c00f-4fac-9cb9-f355d3922feb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:v2=&quot;http://v2.soap2.filiationonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;v2:personSearch>
         &lt;!--Optional:-->
         &lt;personSearch>
            &lt;!--Optional:-->
            &lt;clientAccessCode>a1edeae2a5bd4cde241fdfdb193ca13c&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter>&lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xPerson>
               &lt;!--Optional:-->
               &lt;documentType>&lt;/documentType>
               &lt;!--Optional:-->
               &lt;documentNumber>&lt;/documentNumber>
               &lt;!--Optional:-->
               &lt;tributaryType>&lt;/tributaryType>
               &lt;!--Optional:-->
               &lt;tributaryNumber>&lt;/tributaryNumber>
               &lt;!--Optional:-->
               &lt;lastName>deniard&lt;/lastName>
               &lt;!--Optional:-->
               &lt;name>fernand&lt;/name>
               &lt;!--Optional:-->
               &lt;gender>&lt;/gender>
               &lt;!--Optional:-->
               &lt;level2>&lt;/level2>
               &lt;!--Optional:-->
               &lt;level4>ciudad autonoma buenos aires&lt;/level4>
               &lt;!--Optional:-->
               &lt;postalCode>&lt;/postalCode>
            &lt;/xPerson>
         &lt;/personSearch>
      &lt;/v2:personSearch>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>personSearch</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.PersonSearch_ARG2</defaultValue>
      <description></description>
      <id>9370d00a-e1a9-4e8f-ad06-129e1cf51603</id>
      <masked>false</masked>
      <name>PersonSearch_ARG2</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyElementText(response, 'personSearchResponse.return.status', 'EN')
WS.verifyElementText(response, 'personSearchResponse.return.statusReason', 'SM')
WS.verifyElementText(response, 'personSearchResponse.return.nPerson.numberAlternativePerson', '15')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>89&lt;/documentType>&lt;documentNumber>328737&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>27003287373&lt;/tributaryNumber>&lt;name>MARTA&lt;/name>&lt;lastName>DENIARD&lt;/lastName>&lt;gender>F&lt;/gender>&lt;birthDate>31/01/1930&lt;/birthDate>&lt;street>CNEL JOSE MOLDES&lt;/street>&lt;houseNumber>1871&lt;/houseNumber>&lt;floor>3&lt;/floor>&lt;unit>&lt;/unit>&lt;level2>CAPITAL FEDERAL&lt;/level2>&lt;level4>CIUDAD AUTONOMA BUENOS AIRES&lt;/level4>&lt;postalCode>1428&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>90&lt;/documentType>&lt;documentNumber>4171563&lt;/documentNumber>&lt;tributaryType>80&lt;/tributaryType>&lt;tributaryNumber>20041715635&lt;/tributaryNumber>&lt;name>JORGE LUIS&lt;/name>&lt;lastName>DENIARD&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>15/08/1936&lt;/birthDate>&lt;street>PEDRO MIGUEL ARAOZ&lt;/street>&lt;houseNumber>2490&lt;/houseNumber>&lt;floor>14&lt;/floor>&lt;unit>B&lt;/unit>&lt;level2>CAPITAL FEDERAL&lt;/level2>&lt;level4>CIUDAD AUTONOMA BUENOS AIRES&lt;/level4>&lt;postalCode>1425&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>20956265&lt;/documentNumber>&lt;tributaryType>80&lt;/tributaryType>&lt;tributaryNumber>20209562651&lt;/tributaryNumber>&lt;name>MARTIN JAVIER&lt;/name>&lt;lastName>DENIARD&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>05/08/1969&lt;/birthDate>&lt;street>PEDRO MIGUEL ARAOZ&lt;/street>&lt;houseNumber>2460&lt;/houseNumber>&lt;floor>14&lt;/floor>&lt;unit>B&lt;/unit>&lt;level2>CAPITAL FEDERAL&lt;/level2>&lt;level4>CIUDAD AUTONOMA BUENOS AIRES&lt;/level4>&lt;postalCode>1425&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>18393106&lt;/documentNumber>&lt;tributaryType>80&lt;/tributaryType>&lt;tributaryNumber>27183931062&lt;/tributaryNumber>&lt;name>ANDREA SILVIA&lt;/name>&lt;lastName>DENIARD&lt;/lastName>&lt;gender>F&lt;/gender>&lt;birthDate>18/11/1967&lt;/birthDate>&lt;street>PEDRO MIGUEL ARAOZ&lt;/street>&lt;houseNumber>2490&lt;/houseNumber>&lt;floor>14&lt;/floor>&lt;unit>B&lt;/unit>&lt;level2>CAPITAL FEDERAL&lt;/level2>&lt;level4>CIUDAD AUTONOMA BUENOS AIRES&lt;/level4>&lt;postalCode>1425&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>17635505&lt;/documentNumber>&lt;tributaryType>80&lt;/tributaryType>&lt;tributaryNumber>20176355051&lt;/tributaryNumber>&lt;name>ROBERTO CRISTIAN&lt;/name>&lt;lastName>DENIARD&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>13/02/1966&lt;/birthDate>&lt;street>CIUDAD DE LA PAZ&lt;/street>&lt;houseNumber>1921&lt;/houseNumber>&lt;floor>4&lt;/floor>&lt;unit>S&lt;/unit>&lt;level2>CAPITAL FEDERAL&lt;/level2>&lt;level4>CIUDAD AUTONOMA BUENOS AIRES&lt;/level4>&lt;postalCode>1428&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>90&lt;/documentType>&lt;documentNumber>4294761&lt;/documentNumber>&lt;tributaryType>80&lt;/tributaryType>&lt;tributaryNumber>20042947610&lt;/tributaryNumber>&lt;name>ROBERTO RENATO&lt;/name>&lt;lastName>DENIARD&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>07/08/1939&lt;/birthDate>&lt;street>CNEL JOSE MOLDES&lt;/street>&lt;houseNumber>1865&lt;/houseNumber>&lt;floor>1&lt;/floor>&lt;unit>B&lt;/unit>&lt;level2>CAPITAL FEDERAL&lt;/level2>&lt;level4>CIUDAD AUTONOMA BUENOS AIRES&lt;/level4>&lt;postalCode>1428&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')</verificationScript>
   <wsdlAddress>${PersonSearch_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
