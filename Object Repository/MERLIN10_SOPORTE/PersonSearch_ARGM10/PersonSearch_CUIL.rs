<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PersonSearch_CUIL</name>
   <tag></tag>
   <elementGuidId>b4b3e029-9b4a-4f50-93c6-e2edca61ee4b</elementGuidId>
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
               &lt;tributaryNumber>20254716775&lt;/tributaryNumber>
               &lt;!--Optional:-->
               &lt;lastName>&lt;/lastName>
               &lt;!--Optional:-->
               &lt;name>&lt;/name>
               &lt;!--Optional:-->
               &lt;gender>&lt;/gender>
               &lt;!--Optional:-->
               &lt;level2>&lt;/level2>
               &lt;!--Optional:-->
               &lt;level4>&lt;/level4>
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
WS.verifyElementText(response, 'personSearchResponse.return.nPerson.numberAlternativePerson', '1')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>25471677&lt;/documentNumber>&lt;tributaryType>80&lt;/tributaryType>&lt;tributaryNumber>20254716775&lt;/tributaryNumber>&lt;name>PABLO NORBERTO&lt;/name>&lt;lastName>FERRERO&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>11/11/1976&lt;/birthDate>&lt;street>WASHINGTON&lt;/street>&lt;houseNumber>3414&lt;/houseNumber>&lt;floor>2&lt;/floor>&lt;unit>D&lt;/unit>&lt;level2>CAPITAL FEDERAL&lt;/level2>&lt;level4>CIUDAD AUTONOMA BUENOS AIRES&lt;/level4>&lt;postalCode>1430&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')</verificationScript>
   <wsdlAddress>${PersonSearch_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
