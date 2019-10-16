<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PersonSearch_EN_Nelida</name>
   <tag></tag>
   <elementGuidId>8fa05f47-888a-4ffd-94a0-43115b61a823</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:v2=&quot;http://v2.soap2.enrichment.ases.com/&quot;>
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
               &lt;customAdapter> &lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xPerson>
               &lt;!--Optional:-->
               &lt;documentType> &lt;/documentType>
               &lt;!--Optional:-->
               &lt;documentNumber> &lt;/documentNumber>
               &lt;!--Optional:-->
               &lt;tributaryType> &lt;/tributaryType>
               &lt;!--Optional:-->
               &lt;tributaryNumber>&lt;/tributaryNumber>
               &lt;!--Optional:-->
               &lt;lastName>VILLAR&lt;/lastName>
               &lt;!--Optional:-->
               &lt;name>NELIDA&lt;/name>
               &lt;!--Optional:-->
               &lt;gender> &lt;/gender>
               &lt;!--Optional:-->
               &lt;level2>&lt;/level2>
               &lt;!--Optional:-->
               &lt;level4>CIUDAD AUTONOMA BUENOS AIRES&lt;/level4>
               &lt;!--Optional:-->
               &lt;postalCode> &lt;/postalCode>
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
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>127339&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>27001273391&lt;/tributaryNumber>&lt;name>NELIDA&lt;/name>&lt;lastName>VILLAR&lt;/lastName>&lt;gender>F&lt;/gender>&lt;birthDate>23/07/1928&lt;/birthDate>&lt;street>AVDA DIRECTORIO&lt;/street>&lt;houseNumber>6884&lt;/houseNumber>&lt;floor>1&lt;/floor>&lt;unit> &lt;/unit>&lt;level2>CAPITAL FEDERAL&lt;/level2>&lt;level4>CIUDAD AUTONOMA BUENOS AIRES&lt;/level4>&lt;postalCode>1440&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>3566010&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>27035660106&lt;/tributaryNumber>&lt;name>NELIDA&lt;/name>&lt;lastName>VILLAR&lt;/lastName>&lt;gender>F&lt;/gender>&lt;birthDate>29/01/1937&lt;/birthDate>&lt;street>JUAN ZELARRAYAN&lt;/street>&lt;houseNumber>1806&lt;/houseNumber>&lt;floor>PB&lt;/floor>&lt;unit>B&lt;/unit>&lt;level2>CAPITAL FEDERAL&lt;/level2>&lt;level4>CIUDAD AUTONOMA BUENOS AIRES&lt;/level4>&lt;postalCode>1406&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>1888517&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>23018885174&lt;/tributaryNumber>&lt;name>NELIDA&lt;/name>&lt;lastName>VILLAR&lt;/lastName>&lt;gender>F&lt;/gender>&lt;birthDate>06/04/1935&lt;/birthDate>&lt;street>PJE EL CHASQUE&lt;/street>&lt;houseNumber>6580&lt;/houseNumber>&lt;floor> &lt;/floor>&lt;unit> &lt;/unit>&lt;level2>CAPITAL FEDERAL&lt;/level2>&lt;level4>CIUDAD AUTONOMA BUENOS AIRES&lt;/level4>&lt;postalCode>1408&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>89&lt;/documentType>&lt;documentNumber>1730990&lt;/documentNumber>&lt;tributaryType>80&lt;/tributaryType>&lt;tributaryNumber>27017309906&lt;/tributaryNumber>&lt;name>NELIDA&lt;/name>&lt;lastName>VILLAR&lt;/lastName>&lt;gender>F&lt;/gender>&lt;birthDate>17/10/1933&lt;/birthDate>&lt;street>TRONADOR&lt;/street>&lt;houseNumber>4062&lt;/houseNumber>&lt;floor>PB&lt;/floor>&lt;unit>4&lt;/unit>&lt;level2>CAPITAL FEDERAL&lt;/level2>&lt;level4>CIUDAD AUTONOMA BUENOS AIRES&lt;/level4>&lt;postalCode>1430&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>3681143&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>27036811434&lt;/tributaryNumber>&lt;name>NELIDA&lt;/name>&lt;lastName>VILLAR&lt;/lastName>&lt;gender>F&lt;/gender>&lt;birthDate>11/12/1937&lt;/birthDate>&lt;street>AVDA BERNARDINO RIVADAVIA&lt;/street>&lt;houseNumber>6155&lt;/houseNumber>&lt;floor>6&lt;/floor>&lt;unit>90&lt;/unit>&lt;level2>CAPITAL FEDERAL&lt;/level2>&lt;level4>CIUDAD AUTONOMA BUENOS AIRES&lt;/level4>&lt;postalCode>1406&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>17325&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>27000173253&lt;/tributaryNumber>&lt;name>NELIDA&lt;/name>&lt;lastName>VILLAR&lt;/lastName>&lt;gender>F&lt;/gender>&lt;birthDate>01/03/1930&lt;/birthDate>&lt;street>VICTOR HUGO&lt;/street>&lt;houseNumber>2430&lt;/houseNumber>&lt;floor>3&lt;/floor>&lt;unit>&lt;/unit>&lt;level2>CAPITAL FEDERAL&lt;/level2>&lt;level4>CIUDAD AUTONOMA BUENOS AIRES&lt;/level4>&lt;postalCode>1408&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')</verificationScript>
   <wsdlAddress>${PersonSearch_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
